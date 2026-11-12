use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::process::{Command, ExitCode, Stdio};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CEILING: &str = ".ast-grep/sloppy-ceiling.tsv";
const OUTPUT_MAX: u64 = 8 * 1024 * 1024;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Slop: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).take(5).collect();
    let mode = arguments.first().map(String::as_str).unwrap_or("check");
    if !matches!(mode, "check" | "update" | "init") {
        return Err("expected check, update, or init".into());
    }
    let base = match arguments.as_slice() {
        [] | [_] => None,
        [_, flag, base] if flag == "--base" && !base.starts_with('-') => Some(base),
        _ => return Err("usage: slop [check|update|init] [--base REV]".into()),
    };
    let mut counts = BTreeMap::new();
    for (index, entry) in fs::read_dir(".ast-grep/rules")?.take(65).enumerate() {
        if index == 64 {
            return Err("rule directory exceeds 64 entries".into());
        }
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "yml") {
            let mut source = String::new();
            fs::File::open(path)?
                .take(65537)
                .read_to_string(&mut source)?;
            if source.len() > 65536 {
                return Err("rule file exceeds 64 KiB".into());
            }
            for id in source.lines().filter_map(|line| line.strip_prefix("id: ")) {
                if counts.insert(id.to_owned(), 0_u32).is_some() {
                    return Err(format!("duplicate rule {id}").into());
                }
            }
        }
    }
    if counts.is_empty() || counts.len() > 64 {
        return Err("expected 1 to 64 rules".into());
    }
    let (status, output) = capture(Command::new("ast-grep").args([
        "scan",
        "--config",
        ".ast-grep/sgconfig.yml",
        "--json=compact",
    ]))?;
    if !matches!(status.code(), Some(0 | 1)) {
        return Err(format!("scanner failed: {status}").into());
    }
    let findings: Vec<Finding> = serde_json::from_slice(&output)?;
    for finding in findings {
        let count = counts
            .get_mut(&finding.rule_id)
            .ok_or("scanner returned an unknown rule")?;
        *count = count.checked_add(1).ok_or("match count overflow")?;
        eprintln!(
            "{}:{}:{}: {}: {}",
            finding.file,
            finding.range.start.line + 1,
            finding.range.start.column + 1,
            finding.rule_id,
            finding.message
        );
    }
    let ceiling = if mode == "init" {
        if fs::exists(CEILING)? {
            return Err("ceiling already exists; use update".into());
        }
        None
    } else {
        let mut source = String::new();
        fs::File::open(CEILING)?
            .take(16385)
            .read_to_string(&mut source)?;
        let ceiling = parse_ceiling(&source)?;
        compare(&counts, &ceiling, mode == "check")?;
        Some(ceiling)
    };
    if let Some(base) = base {
        let (status, paths) =
            capture(Command::new("git").args(["ls-tree", "--name-only", base, "--", CEILING]))?;
        if !status.success() {
            return Err("cannot resolve baseline revision".into());
        }
        if !paths.is_empty() {
            let (status, previous) =
                capture(Command::new("git").args(["show", &format!("{base}:{CEILING}")]))?;
            if !status.success() {
                return Err("cannot read baseline ceiling".into());
            }
            let previous = parse_ceiling(std::str::from_utf8(&previous)?)?;
            let current = match &ceiling {
                Some(ceiling) if mode == "check" => ceiling,
                _ => &counts,
            };
            compare(current, &previous, false)?;
        }
    }
    if mode != "check" {
        let contents: String = counts
            .iter()
            .map(|(id, count)| format!("{id}\t{count}\n"))
            .collect();
        fs::write(CEILING, contents)?;
    }
    println!("Slop: {} rules checked", counts.len());
    Ok(())
}

fn capture(command: &mut Command) -> Result<(std::process::ExitStatus, Vec<u8>)> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let mut output = Vec::new();
    let read = (|| -> std::io::Result<usize> {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::other("missing child stdout"))?;
        stdout.take(OUTPUT_MAX + 1).read_to_end(&mut output)
    })();
    if read.is_err() || output.len() as u64 > OUTPUT_MAX {
        let killed = child.kill();
        let reaped = child.wait();
        return Err(format!(
            "bounded command output failed: {read:?}; kill: {killed:?}; wait: {reaped:?}"
        )
        .into());
    }
    Ok((child.wait()?, output))
}

fn parse_ceiling(source: &str) -> Result<BTreeMap<String, u32>> {
    if source.len() > 16 * 1024 {
        return Err("ceiling exceeds 16 KiB".into());
    }
    let mut counts = BTreeMap::new();
    for line in source.lines() {
        let (id, count) = line.split_once('\t').ok_or("expected rule<TAB>count")?;
        if id.is_empty()
            || !id
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        {
            return Err("invalid rule id".into());
        }
        if counts.insert(id.to_owned(), count.parse()?).is_some() {
            return Err("duplicate ceiling rule".into());
        }
    }
    if counts.is_empty() || counts.len() > 64 {
        return Err("expected 1 to 64 ceiling rows".into());
    }
    Ok(counts)
}

fn compare(
    current: &BTreeMap<String, u32>,
    previous: &BTreeMap<String, u32>,
    exact: bool,
) -> Result<()> {
    for (id, ceiling) in previous {
        let count = current
            .get(id)
            .ok_or_else(|| format!("missing rule {id}"))?;
        if count > ceiling {
            return Err(format!("{id}: {count} exceeds ceiling {ceiling}").into());
        }
        if exact && count < ceiling {
            return Err(format!(
                "{id}: decreased to {count}; run slop update and commit the lower ceiling"
            )
            .into());
        }
    }
    if exact && current.keys().ne(previous.keys()) {
        return Err("rule inventory changed; initialize ceilings for new rules explicitly".into());
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Finding {
    file: String,
    message: String,
    rule_id: String,
    range: FindingRange,
}
#[derive(Deserialize)]
struct FindingRange {
    start: Position,
}
#[derive(Deserialize)]
struct Position {
    line: u32,
    column: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ceilings_reject_increases_missing_rules_and_unrecorded_reductions() {
        let old = parse_ceiling("rust-expect\t2\n").unwrap();
        for count in 0..4 {
            let current = parse_ceiling(&format!("rust-expect\t{count}\n")).unwrap();
            assert_eq!(compare(&current, &old, false).is_ok(), count <= 2);
            assert_eq!(compare(&current, &old, true).is_ok(), count == 2);
        }
        assert!(compare(&BTreeMap::new(), &old, false).is_err());
        assert!(parse_ceiling("rust-expect\t1\nrust-expect\t2\n").is_err());
        assert!(parse_ceiling("rust-expect\t-1\n").is_err());
        assert!(parse_ceiling("").is_err());
    }
}

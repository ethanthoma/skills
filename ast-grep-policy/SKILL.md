---
name: ast-grep-policy
description: Build, maintain, and verify ast-grep structural lint rules and downward-only finding baselines. Use for executable coding policies, rule fixtures, or adopting a lint debt ratchet in a repository.
---

# ast-grep policy

Turn selected repository policies into tested structural checks. Preserve the
repository's language choices, tooling, and source boundaries. Use `rg` for plain
text discovery. Use ast-grep when syntax or ancestry changes what should match.

## Adopt or maintain

1. Read repository guidance and existing scanner configuration, CI, and formatter
   settings. Reuse an existing setup instead of adding a second one.
2. Identify the requested policies and production source paths. Keep generated,
   vendored, and test code separate where policy requires it. Do not assume this
   skill's example paths or bans apply to every repository.
3. Prefer syntax nodes, fields, patterns, and ancestry over whole-file regexes.
   Give every YAML rule document a stable unique ID and an actionable message.
4. Add matching and nonmatching fixtures for each rule. Include nested syntax,
   legitimate lookalikes, and test exclusions where relevant. Run the fixtures
   before measuring repository findings.
5. Inspect actual diagnostics before adopting a baseline. Fix false positives by
   correcting the intended selector. Do not broaden exclusions to hide debt.
6. For incremental enforcement, use the contract and adoption instructions in
   [references/ratchet.md](references/ratchet.md). For an already clean policy,
   a direct scan that fails on findings may be sufficient.
7. Run the project's formatter, rule tests, checker tests, and repository check.
   Commit fixes and baseline reductions together. Report the selected scope,
   checks run, remaining debt, and relevant coverage limits.

## Reusable assets

`assets/ast-grep/` contains an example config, seven Rust rules, three browser
JavaScript rules, fixtures, and a shared Rust test-item matcher. Copy selected
files into the target's `.ast-grep/` and adapt paths and exclusions. These are
policy examples, not universal bans. Remove fixtures for rules not adopted.

Rust examples flag `unwrap`, `expect`, abort/stub macros, empty wildcard arms,
`let _ = ...`, deferral comments, and machine-specific paths. Browser examples
flag empty catches, deferral comments, and machine-specific paths. Review these
semantics before adoption:

- `rust-discard-result` matches every `let _ = ...`, without type information.
- Deferral-comment rules flag words such as `temporary` even in justified notes.
- Test ancestry recognizes selected attribute forms, not arbitrary `cfg` logic
  or every framework. Extend fixtures before broadening the matcher.
- Macro token bodies may not expose ordinary expression nodes. These checks
  supplement compiler and type-aware linting.

`assets/ratchet/slop.rs` is a Rust checker template with comparison tests.
Read the ratchet reference before copying it. Do not copy another project's
numeric baseline, CI runner configuration, deployment steps, or dependencies
unrelated to this checker.


---
name: gitignore-allowlist
description: >
  Maintain inverted, default-deny .gitignore files. Use when creating,
  editing, reviewing, or debugging .gitignore rules, when adding source
  files or assets that Git unexpectedly ignores, or when checking which
  repository files should be tracked. Prefer explicit allowlists over
  conventional blocklists. Preserve narrow exceptions and verify
  behavior with Git.
---

# Gitignore Allowlist

## Philosophy

Use an inverted `.gitignore` by default:

```gitignore
*
!*/
```

Everything is ignored unless explicitly allowed.

The repository should track intentional source code, configuration,
documentation, tests, and authored assets without automatically
including generated files, temporary files, local state, or secrets.

Treat `.gitignore` as an explicit declaration of repository contents.

## 1. Preserve the default-deny policy

When creating or modifying `.gitignore`:

- Begin with `*` and `!*/`.
- Explicitly allowlist files and file patterns.
- Preserve existing exceptions unless removal is requested or justified.
- Group related exceptions under concise comments.
- Prefer narrow paths and extensions over broad directory exceptions.
- Do not replace the inverted policy with a conventional blocklist.
- Do not introduce redundant ignore rules already covered by `*`.

Example:

```gitignore
*
!*/

# Repository metadata
!/.gitignore
!/README.md
!/LICENSE

# Rust
!/Cargo.toml
!/Cargo.lock
!/src/**/*.rs
!/tests/**/*.rs

# Nix
!/flake.nix
!/flake.lock
!/nix/**/*.nix
```

## 2. Allowlist intentional repository content

Before adding an exception, determine whether the file is:

- Authored source code.
- Required configuration.
- Documentation.
- Tests or test fixtures.
- A reproducibility-critical lockfile.
- An intentional binary or media asset.
- Generated output.
- Local development state.
- A secret or credential.

Allowlist the first six categories when appropriate.

Keep generated output and local state ignored unless the repository
deliberately versions them.

Never allowlist credentials or private configuration automatically.

## 3. Prefer narrow exceptions

Good:

```gitignore
!/src/**/*.rs
!/assets/css/**/*.css
!/assets/imgs/**/*.svg
!/tests/fixtures/**/*.json
```

Riskier:

```gitignore
!/src/**
!/assets/**
!/tests/**
```

Broad exceptions can unintentionally admit generated files, temporary
artifacts, or sensitive data.

Use broad patterns only when the directory is intentionally
fully version-controlled and its contents are understood.

Prefer extension-based patterns for homogeneous source directories.

Prefer explicit filenames for configuration and infrastructure files.

## 4. Handle directory traversal correctly

Git cannot re-include a file when an ancestor directory remains
excluded.

The initial `!*/` permits traversal into directories while leaving
files ignored by default.

Do not remove it without understanding the effect on nested exceptions.

Patterns using `**` should be tested against representative nested
paths.

Nested `.gitignore` files may introduce additional rules. Account
for those rules when diagnosing ignored files.

## 5. Keep new files trackable

When adding a file that should be committed:

1. Check whether an existing allowlist rule covers it.
2. If not, add the narrowest reasonable exception.
3. Group it with related rules.
4. Verify that Git recognizes the file as eligible for tracking.
5. Check that the new rule does not admit unintended files.

Do not broaden an entire directory merely to admit one file.

Do not silently leave newly created source files ignored.

## 6. Verify with Git

Use Git's own matching behavior rather than assuming a glob works.

Inspect an individual file:

```bash
git check-ignore -v path/to/file
```

Inspect ignored and untracked files:

```bash
git status --short --untracked-files=all
git status --short --ignored
```

Inspect tracked files:

```bash
git ls-files
```

Use `git check-ignore -v --no-index` when examining patterns that
affect already tracked files.

Remember:

- An ignored file can still be tracked if it was committed earlier.
- Changing `.gitignore` does not automatically untrack existing files.
- `git check-ignore` can report matching negation patterns.
- `git status` is needed to confirm whether an untracked file is
  actually eligible for addition.

Do not automatically run `git rm --cached` or change the index merely
because a tracked file matches an ignore rule.

## 7. Review security-sensitive exceptions

Explicit exceptions are not proof that a file is safe to commit.

Pay particular attention to:

- `.env` files.
- Private keys and credentials.
- Terraform state and variable files.
- Deployment secrets.
- Local machine configuration.
- Generated configuration containing tokens.

For example:

```gitignore
!/secrets/prod.yaml
```

This may be intentional if the file contains encrypted secrets, such
as a SOPS-managed document.

Do not assume it is safe merely because it is allowlisted.

Inspect the repository's encryption and secret-management conventions
before recommending or modifying such exceptions.

Never display secret values in the final response.

## 8. Minimize rule complexity

Prefer a short, readable allowlist organized by purpose.

Avoid:

- Duplicated exceptions.
- Overlapping rules with unclear precedence.
- Repeated negation and re-ignoring sequences.
- Large enumerations when a safe glob expresses the same intent.
- Broad exceptions followed by extensive corrective blocklists.

When simplifying rules, verify that the set of eligible files remains
unchanged unless a change is intentional.

Optimize for clarity and predictable tracking behavior rather than
the fewest possible lines.

## 9. Scope and behavior

Apply this skill when:

- Creating a new repository.
- Modifying `.gitignore`.
- Adding new file types or directories.
- Investigating missing files in commits.
- Reviewing repository hygiene.
- Refactoring source or asset layouts.

For unrelated coding tasks, do not perform a repository-wide
`.gitignore` audit.

When adding source files, check that they are trackable if the
repository uses this convention.

Do not modify `.gitignore` unnecessarily.

## 10. Reporting

For routine changes, briefly report:

- Which exceptions were added, changed, or removed.
- Whether Git verification succeeded.
- Any potentially sensitive files requiring attention.

Do not reproduce the entire `.gitignore` unless requested.

## Guiding principle

Ignore everything by default.

Explicitly admit intentional repository content.

Verify the resulting tracked-file boundary with Git.

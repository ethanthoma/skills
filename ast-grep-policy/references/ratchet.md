# Downward-only findings

The baseline is a sorted TSV of `rule-id<TAB>count`, including zero-count rules.
Generate it from the target repository after reviewing findings. Never copy the
example repository's debt counts.

## Checker contract

- `init` creates a baseline once and refuses to overwrite it.
- `check` requires the current rule inventory and counts to match the baseline.
  Increases fail. Reductions also fail until recorded with `update`.
- `update` allows reductions and new rule IDs, but rejects increases for existing
  IDs and removal of existing IDs. Review new-rule baselines explicitly.
- `check --base REV` additionally compares the current baseline against the Git
  revision's baseline. Existing IDs cannot disappear or increase. A valid
  revision without a baseline permits first adoption. An invalid revision fails.
- Scanner errors, malformed output, unknown diagnostic IDs, duplicate IDs,
  invalid counts, and missing rules fail the check. Print findings with source
  locations and remediation messages even when existing debt is allowed.

Counts constrain net debt per rule. They do not prevent replacing one violation
with another or weakening a selector. Rule, exclusion, and CI changes need review.

## Adopt the Rust template

The template deliberately uses a fixed repository layout rather than a general
configuration API. It expects execution from the repository root, `.ast-grep/`
configuration, and `.ast-grep/sloppy-ceiling.tsv`. Keep this convention or update
all path uses together when the target requires another layout.

Copy `assets/ratchet/slop.rs` into the target Rust tooling, commonly
`src/bin/slop.rs`. It requires `serde` with the `derive` feature and `serde_json`.
Reuse existing dependencies and lockfile. For a non-Rust application, place it
in a small separate Rust tool crate rather than changing the application's
implementation language. Do not introduce a Rust toolchain solely for a direct
zero-debt scan that the existing build can already enforce.

The template inventories only direct `.yml` files under `.ast-grep/rules/`.
Every rule ID must occupy its own unindented `id: lowercase-hyphenated-name`
line, with no quoting or trailing comment. Its line-based inventory is not a
general YAML parser. Keep this narrow convention for the supplied assets. If
the target needs arbitrary YAML syntax or nested rule directories, replace the
inventory with structured parsing and test zero-finding rule discovery too.

Resource limits are 64 directory entries, 64 rules, 64 KiB per rule file,
16 KiB baseline, and 8 MiB captured command output. Scanner and Git processes
have no internal timeout. Use a bounded CI job timeout and consider subprocess
timeouts if the target needs stronger guarantees. Bounds are guardrails, not a
reason to silently drop diagnostics.

Supply ast-grep through the repository's pinned tooling. On Nix, add
`pkgs.ast-grep` to the development shell. For a root Rust binary, typical commands
are:

```sh
nix develop --command ast-grep test --config .ast-grep/sgconfig.yml --skip-snapshot-tests
nix develop --command cargo test --locked --bin slop
nix develop --command cargo run --locked --bin slop -- init
nix develop --command cargo run --locked --bin slop -- check
nix develop --command cargo run --locked --bin slop -- update
nix develop --command cargo run --locked --bin slop -- check --base origin/main
```

Use `init` only on first adoption and `update` after reviewed reductions or rule
additions. Do not run them automatically just to make a failed check pass.

## CI and verification

Run rule fixtures, checker tests, and `check --base REV` in CI. Fetch the required
history. Select the PR base revision for PRs and the previous revision for pushes
from the CI event. Do not substitute HEAD after failing to resolve a supplied
base. Handle an initial push's null revision explicitly. Adapt the runner and
event configuration to the target rather than copying a site's workflow.

Verify in a temporary repository with representative production paths:

1. Bootstrap and confirm an unchanged scan passes.
2. Add a violation and confirm check and update reject the increase.
3. Remove a violation and confirm check requires an update, then passes after it.
4. Increase both findings and the edited baseline and confirm the Git-base check
   rejects the increase.
5. Exercise invalid revisions and malformed baseline input. Confirm failure is
   attributable to the intended condition, not a missing tool or broken fixture.

Keep temporary repositories and generated build files outside the skill package.
Never weaken the rule or baseline to repair a scanner execution failure.

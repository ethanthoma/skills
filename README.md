# Skills

Personal agent skills. Each top-level skill directory contains a `SKILL.md` entrypoint
and any supporting references.

```sh
git clone https://github.com/ethanthoma/skills.git
```

Link individual skill directories into your agent's supported skills directory.
Do not replace a populated skills directory: it may contain skills managed by other tools.

My Home Manager configuration maintains a checkout at `~/.local/share/skills` and
links selected skills into `~/.config/agents/skills`, `~/.config/codex/skills`, and
`~/.config/pi/skills`. Activation clones a missing checkout but does not pull updates.
Edit and commit in the checkout; review upstream changes before pulling on other machines.
Start a new agent session after changing skills.

## Attribution

The plain-technical-prose skill credits its upstream inspiration in its entrypoint.
Its vendored reference checklist includes the upstream MIT license in
`plain-technical-prose/references/LICENSE`. That license applies to the vendored
material, not automatically to the entire repository.

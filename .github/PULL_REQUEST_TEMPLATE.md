## What and why

<!-- What does this change, and why? Link the issue: "Fixes #123". -->

## Checklist

- [ ] Tests added or updated (a bug fix includes a test that fails without it)
- [ ] `scripts/check.sh` passes
- [ ] `MAYU_RUN_SYSTEMD_TESTS=1 cargo test` passes locally (if backends or enforcement changed; output below)
- [ ] `README.md` updated if behaviour, flags, env vars or exit codes changed
- [ ] `CHANGELOG.md` has a line under *Unreleased*
- [ ] Follows [AGENTS.md](../AGENTS.md) (invariants and code quality)

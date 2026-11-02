# Contributing

Thanks for helping. Bug reports, docs fixes and code are all welcome. Coding agents: read
[AGENTS.md](AGENTS.md) too.

## Before you start

- **Bugs:** open an issue with the mayu version (`mayu --version`), OS, kernel (`uname -r`), systemd version
  (`systemctl --version`) and a minimal reproduction.
- **Features and larger changes:** open an issue first so we can agree on the design before you write code.
- **Security issues:** don't open a public issue. See [SECURITY.md](SECURITY.md).

## Repository layout

| Path | What it is |
|---|---|
| `src/lib.rs` | The runner, as a library. |
| `src/main.rs` | The `mayu` binary. |
| `src/bin/mayu-run.rs` | The same program installed as `mayu-run`. |
| `tests/` | End-to-end tests that run the binaries. |
| `scripts/check.sh` | fmt, clippy and tests: what CI runs. |

## Setup

You need Rust 1.85 or newer. Enforcement and its tests need Linux with cgroup v2 and a systemd user manager;
everything else builds and tests on any Unix.

## Running checks

```bash
scripts/check.sh                          # fmt, clippy (warnings denied), tests: same as CI
MAYU_RUN_SYSTEMD_TESTS=1 cargo test       # also run the tests that start real systemd user units
```

The systemd tests start transient units in a throwaway `mayu-test-<pid>.slice`, never in your real
`mayu.slice`, and remove it afterwards. CI can't run them (no user manager), so run them locally when you change
backends or enforcement, and paste the output in the PR.

## Commits and pull requests

See [AGENTS.md](AGENTS.md): commit messages are `area: what changed`, one logical change per PR, every change
ships with tests, and user-visible changes update `README.md` and `CHANGELOG.md`.

# AGENTS.md

Guidance for coding agents (and humans) working on this repo. Read [CONTRIBUTING.md](CONTRIBUTING.md) for setup
and test commands.

## What this is

The `mayu` crate: a Rust binary that runs commands under a shared per-user CPU and memory budget, so parallel
coding agents on one machine don't thrash it. A per-user daemon admits or queues each command. The command runs
under a small shim inside a transient systemd user unit in `mayu.slice`, and the kernel enforces the budget
through cgroup v2. Subcommands the binary doesn't own are forwarded to the Python `mayu` CLI
([Siddid-Soni/mayu](https://github.com/Siddid-Soni/mayu)), which this repo does not depend on.

## Invariants: don't break these

- **Never kill by default.** Enforcement throttles, reclaims and pauses. A command is killed only on an explicit
  `mayu cancel`, an explicit `--timeout`, `--mem-hard`, or when the caller marked it `--killable`. No code path
  may kill a command that didn't opt in.
- **Never lose a command.** A queued command is on disk before the client is told it is queued. A running
  command survives the client dying and the daemon dying or restarting, and the daemon re-adopts it on start.
  Recovery must neither lose a command nor start it twice.
- **The daemon is a scheduler, not a parent.** Nothing correctness-critical lives only in the daemon's memory or
  as a child-process handle of the daemon. It can be killed or upgraded at any moment.
- **One writer per file, atomic writes.** `state` is written only by the daemon; `status` and `log` only by the
  shim. Every state-file write is tmp + fsync + rename.
- **Output never sits in memory.** The shim and the client stream through fixed-size buffers. No
  `read_to_end` / `read_to_string` on a log or a pipe; reading the tail of a log seeks from the end.
- **No fork from a threaded process.** Spawn with `std::process::Command` or tokio. The daemon and the shim are
  separate executions of this binary.
- **Tests never touch the real `mayu.slice`.** Tests that start units use their own slice
  (`MAYU_RUN_SLICE=mayu-test-<pid>.slice`) and remove it. Slice properties are always set with
  `systemctl --user set-property --runtime`, never persisted.
- **Same user only.** The socket is 0600 in a 0700 directory and the daemon checks the peer's uid
  (`SO_PEERCRED`). A command's spec can hold secrets (its environment): 0600, erased when no longer needed.
- **Versioned protocol and state.** A new daemon reads state written by the previous version, or refuses it with
  a clear message. It never silently rewrites it.

## Code quality

- **Match the surrounding code**: naming, comment density, idioms. Comments explain *why*, not *what*.
- **Errors:** a `thiserror` error type. No `unwrap()`, `expect()` or panics outside tests and truly impossible
  paths.
- **`unsafe`** (libc calls) only with a `// SAFETY:` comment; clippy's `undocumented_unsafe_blocks` enforces it.
- **fmt and clippy clean**, with warnings denied: `scripts/check.sh` must pass.
- **Dependencies:** keep the list short. Each new one needs a reason in the PR: the binary is compiled by
  `cargo install` on developer machines, so build time and supply-chain surface matter.
- **Linux first.** macOS must still build and run without enforcement (the `plain` backend).
- **Docs:** user-visible behaviour, flags, env vars and exit codes are documented in `README.md` in the same
  change. Add a line to `CHANGELOG.md` under *Unreleased*.

## Tests

- Every change ships with tests. A bug fix starts with a test that fails without the fix.
- Unit tests go next to the code; end-to-end tests that run the binary go in `tests/`. Scheduling and pressure
  policy are pure functions tested without a daemon.
- Tests that need a real systemd user manager run only with `MAYU_RUN_SYSTEMD_TESTS=1` (not in CI).
- Prefer waiting on a condition with a timeout over fixed sleeps.
- Before saying a change is done, run `scripts/check.sh` (plus the systemd tests when touching backends or
  enforcement) and report the actual output.

## Commits and PRs

- Commit messages: `area: what changed` (`feat(shim): ...`, `fix(daemon): ...`, `docs: ...`).
- One logical change per PR. Don't commit `target/` or local planning notes (`docs/superpowers/`,
  `.superpowers/`).

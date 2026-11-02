# mayu command runner

Run heavy commands under a shared CPU and memory budget, so several coding agents on one machine (Claude Code,
Codex, ...) can build and test in parallel without thrashing it or getting their work killed by the OOM killer.

```bash
mayu run --cpu 8 --mem 8G -- cargo test
```

> **Status: early development.** Nothing works yet beyond `mayu --version`. The design below is what is being
> built; this README will say what actually ships, release by release.

## What it will do

- **Admit or queue.** A per-user daemon starts a command when the budget has room and queues it otherwise.
  A request bigger than the whole budget fails at once instead of hanging.
- **Enforce without killing.** On Linux with systemd and cgroup v2, each command runs in its own transient user
  unit. The kernel throttles, reclaims and (for commands you mark `--pausable`) pauses them. mayu never kills a
  command unless you ask for it: `mayu cancel`, `--timeout`, `--mem-hard` or `--killable`.
- **Survive anything short of a reboot.** A running command outlives the agent's tool timeout, the client dying
  and the daemon restarting. Any client can re-attach with `mayu wait ID` or read `mayu logs ID`.
- **Give agents output they can use.** Output streams to disk through a small fixed buffer and is never held in
  memory. An agent sees the start, the end, the exact exit code and a one-line footer with peak memory, timings
  and the log path, not a flood that fills its context.

Without systemd or cgroup v2 (macOS, containers), mayu still queues commands and keeps durable logs and exit
status, but enforces nothing.

## Relationship to the mayu task queue

[mayu](https://github.com/Siddid-Soni/mayu) is a Redis-backed Python task queue with a `mayu` command of its own.
This binary is also called `mayu`: it handles `run`, `wait`, `logs`, `status`, `cancel`, `daemon`, `doctor` and
`hook`, and forwards any other subcommand (`mayu worker`, `mayu inspect`, ...) to the Python CLI. It also installs
as `mayu-run`, the same program under a name a Python virtualenv's `mayu` can't shadow.

## Install

Not published yet. To try the development version:

```bash
cargo install --git https://github.com/Siddid-Soni/mayu-run
```

Once released: `cargo install mayu`.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your
option.

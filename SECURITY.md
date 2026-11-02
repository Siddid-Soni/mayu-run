# Security policy

Please report vulnerabilities privately through
[GitHub security advisories](https://github.com/Siddid-Soni/mayu-run/security/advisories/new),
not in a public issue. Include the affected version, a description and, if you can, a reproduction.
You'll get an acknowledgement within a few days.

Only the latest release receives security fixes while mayu is pre-1.0.

What to expect from the design: mayu runs commands as the user who asks for them, through a per-user daemon on a
Unix socket that only that user can open (0600 in a 0700 directory, peer uid checked). Anything that can open
the socket can run commands as you. Commands started through the daemon run under the systemd user manager, so
they run **outside** a calling agent's sandbox; a sandboxed caller that must stay sandboxed uses `--inline`.
A command's saved spec includes its environment, which can hold secrets; it is readable only by you and is
erased when no longer needed.

# Security Policy — RapidTally Ace

## Supported versions

| Version | Supported |
|---|---|
| 0.1.x | yes |

## Reporting a vulnerability

Please open a private security advisory via GitHub
(*Security → Report a vulnerability*) or contact the repository owner
directly. Do **not** open a public issue for security reports.

Include: affected version/commit, platform, reproduction steps, and impact.

## Scope notes specific to this project

- **Never attach real accounting data** (`.QBW`/`.QBB` files, exports,
  screenshots of real books) to bug reports or issues. Use synthetic files.
- The phase-0 probe reads files read-only and sends nothing anywhere; it has
  no network code at all. Reports about unexpected file writes or network
  activity are high priority.
- Binaries are produced exclusively by the pinned GitHub Actions workflows;
  verify downloads against `SHA256SUMS.txt` on the release page.

# Roadmap & decision gates — RapidTally Ace

Legend: **done** / in progress / not started / **blocked**.

| Phase | Scope | Exit gate | Status |
|---|---|---|---|
| Phase 0 — scaffold | Rust workspace, format-probe harness, clean-room protocol, multi-platform release engineering (Windows 7+ x86/x64, macOS Catalina+, Android Oreo+ armv7/arm64, Linux Mint/Debian + Flatpak) | public v0.1.0 release | **done (v0.1.0)** |
| G0 — legal opinion | Written counsel review of the clean-room plan (US fair use / DMCA §1201(f), EU 2009/24/EC; Sega v. Accolade, Oracle v. Google frames) **before** any experiment on real `.QBW` files beyond trivial sniffing | G0 memorandum approved | not started (blocking) |
| G1 — read-path fidelity | Parse SQL Anywhere pages and catalog from legally obtained samples; fidelity target ≥ 95 % across a ground-truth corpus; de-obfuscation experiments recorded in the evidence log | G1 report | not started |
| Phase 1 — vertical slice | Chart of accounts, journal entry, general ledger + trial balance report, CSV/QIF export | end-to-end demo on real books | not started |
| Phase 2 — desktop shell | Qt 5.15 LTS with an original Material-3-style skin; Windows 7+ / macOS Catalina+ / Linux; 1:1 information architecture (not 1:1 trade dress) | moderated usability test | not started |
| Phase 3 — mobile shell | Kotlin/Compose, Material 3, armv7 + arm64, minSdk 26 (Android 8.0 Oreo) | internal alpha | not started |
| Phase 4 — hardening | `.QBB` backup read path, multi-currency, packaging polish (Flathub submission), auto-update story | public beta | not started |

## Naming history

The project was internally codenamed **Bookwright** during the feasibility
blueprint stage and was renamed to **RapidTally Ace** at the first public
release (v0.1.0, 2026-11-02), with a new project logo provided by the owner.

## Toolchain policy

- **Rust 1.77.x pinned** (`rust-toolchain.toml`): Rust 1.78 dropped Windows 7
  as a supported host; the Windows 7 SP1 floor is a hard product requirement.
- **Qt 5.15** (phase 2) is the last Qt line supporting Windows 7.
- **Android minSdk 26** (Oreo): required by the target device matrix and
  consistent with the Play 64-bit policy; both `armeabi-v7a` and
  `arm64-v8a` are built.

## Immediate next steps (phase 0 → G1)

1. Draft the G0 legal memorandum (counsel review).
2. Collect a legally owned ground-truth corpus of `.QBW`/`.QBB` samples and
   record baseline probe reports in `docs/evidence/`.
3. Extend `rapidtally-format` from heuristics to a verified page parser
   (fidelity harness with a machine-readable report).

# Changelog

All notable changes to RapidTally Ace are documented here.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
versioning: SemVer, with phase labels.

## [0.1.0] — 2026-11-02

### Added
- Initial public release of the **phase-0** workspace (clean-room QuickBooks
  Desktop read-path spike):
  - `rapidtally-format` crate: ASCII marker scan, candidate page-size
    analysis, per-region Shannon-entropy map, hex dump, minimal C ABI
    (`rtace_version`, `rtace_probe_buffer`) for future Kotlin/JNI and Qt
    embedding.
  - `rapidtally-ace` CLI with `inspect`/`probe` commands.
  - Clean-room engineering protocol (`docs/CLEANROOM.md`), evidence-log
    scaffold, roadmap with G0/G1 decision gates.
  - Multi-platform release engineering: Windows 7+ (x86_64 + i686,
    static CRT), macOS Catalina+ (x86_64 + universal2, ad-hoc signed),
    Android 8.0 Oreo+ (arm64-v8a + armeabi-v7a via NDK, API 26),
    Linux Mint/Debian (static musl binary + `.deb`), Flatpak bundle
    (`com.rapidtallyace.RapidTallyAce`), SHA256SUMS on every release.
  - CI on Linux/Windows/macOS with the toolchain pinned to Rust 1.77.2
    (the last release targeting Windows 7).

### Changed
- Project renamed from the internal codename **Bookwright** to
  **RapidTally Ace**; new logo integrated (banner, icons, Flatpak icon).

[0.1.0]: https://github.com/maniesh-NR/RapidTallyAce/releases/tag/v0.1.0

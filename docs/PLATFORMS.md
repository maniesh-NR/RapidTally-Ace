# Platforms, artifacts & caveats — RapidTally Ace v0.1.0

All artifacts are produced by the pinned release workflow
(`.github/workflows/release.yml`) and attached to the GitHub release with a
`SHA256SUMS.txt`. Toolchain: Rust **1.77.2** on every platform.

## Matrix

| Platform | Artifact | Notes |
|---|---|---|
| Windows 7 SP1+ x64 | `rapidtally-ace-0.1.0-windows-x86_64.zip` | CRT statically linked (`crt-static`); no VC++ redistributable needed |
| Windows 7 SP1+ x86 (32-bit) | `rapidtally-ace-0.1.0-windows-i686.zip` | Same static-CRT treatment; 512 MiB probe input cap |
| macOS 10.15 Catalina+ (Intel) | `rapidtally-ace-0.1.0-macos-x86_64.zip` | Ad-hoc signed; see Gatekeeper note |
| macOS 11+ (Apple Silicon / Intel) | `rapidtally-ace-0.1.0-macos-universal2.zip` | Universal binary (`lipo` of x86_64 + aarch64) |
| Android 8.0 Oreo (API 26)+ armv8a | `rapidtally-ace-0.1.0-android-arm64-v8a.zip` | Validation artifact: CLI binary + JNI `.so` (`librapidtally_format.so`) |
| Android 8.0 Oreo (API 26)+ armv7 | `rapidtally-ace-0.1.0-android-armeabi-v7a.zip` | Same, for 32-bit devices |
| Linux (any distro, x86_64) | `rapidtally-ace-0.1.0-linux-x86_64.tar.gz` | Fully static musl build; zero runtime dependencies |
| Linux Mint / Debian / Ubuntu | `rapidtally-ace_0.1.0_amd64.deb` | Built from the static binary |
| Linux (any distro) | `RapidTallyAce-0.1.0-x86_64.flatpak` | Runtime `org.freedesktop.Platform 23.08` |
| All | `SHA256SUMS.txt` | Verify before running |

**Linux Mint note:** Cinnamon, Xfce and MATE are desktop-environment editions
of the same Ubuntu/Debian base — the single static binary and the `.deb`
cover all current Mint variants (20, 21, 22.x) and LMDE, with or without
Flatpak.

## Caveats

### Windows 7+
- The Rust toolchain is pinned to 1.77.2 because Rust 1.78 dropped Windows 7.
- The C runtime is statically linked, so the `.exe` is self-contained.
- If SmartScreen warns about an unsigned binary, choose *More info → Run
  anyway* or verify the hash against `SHA256SUMS.txt`.

### macOS Catalina+
- Binaries are **ad-hoc signed** (no Apple Developer ID). On first launch
  Gatekeeper may block them: run
  `xattr -d com.apple.quarantine rapidtally-ace`, or right-click → *Open*.
- The `universal2` build needs macOS 11+; the `x86_64` build covers Catalina.

### Android Oreo+
- Phase-0 artifacts prove the cross-toolchain (they contain the CLI binary
  and the C-ABI library). They are **not** a user-facing app; the
  Kotlin/Compose shell arrives in phase 3 (see `ROADMAP.md`).
- For a quick check on-device: install Termux, push the binary and run
  `./rapidtally-ace --version`.

### Linux Mint / Debian
- The musl static binary runs on glibc or musl systems alike.
- `.deb`: `sudo apt install ./rapidtally-ace_0.1.0_amd64.deb` (installs
  `/usr/bin/rapidtally-ace` and docs).
- Flatpak: `flatpak install --user ./RapidTallyAce-0.1.0-x86_64.flatpak`
  then `flatpak run com.rapidtallyace.RapidTallyAce inspect <file>`.

## Building locally

```bash
# any OS with rustup (toolchain auto-selected from rust-toolchain.toml)
cargo test --workspace
cargo build --release
```

Cross-target examples used by CI:

```bash
# Windows 32-bit (from windows-2022 runner)
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

# Linux fully static
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Android (needs NDK + cargo-ndk)
cargo install cargo-ndk --locked
cargo ndk --platform 26 --target arm64-v8a --target armeabi-v7a build --release
```

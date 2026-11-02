# Android — phase 0 (validation artifacts)

Phase 0 proves the native toolchain for Android; the user-facing app
(Kotlin/Compose, Material 3) lands in phase 3 per `docs/ROADMAP.md`.

## What the release workflow produces

For each of `arm64-v8a` (armv8a) and `armeabi-v7a` (armv7), built with
NDK r26d at `--platform 26` (Android 8.0 Oreo):

- `rapidtally-ace` — the CLI binary (runs on-device under Termux or `adb shell`)
- `jniLibs/<abi>/librapidtally_format.so` — the C-ABI library
  (`rtace_version`, `rtace_probe_buffer`) ready for JNI embedding

## Reproduce locally

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-ndk --locked
cargo ndk --platform 26 --target arm64-v8a --target armeabi-v7a --output jniLibs build --release
```

(Requires the Android NDK; `ANDROID_NDK_HOME` must point at it.)

## Quick on-device check (Termux)

```bash
# push the zip, extract, then:
chmod +x rapidtally-ace
./rapidtally-ace --version
./rapidtally-ace inspect /sdcard/Download/your-file.qbw
```

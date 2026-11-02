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
export ANDROID_NDK_HOME=/path/to/android-ndk-r26d
NDKBIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
export PATH="$NDKBIN:$PATH"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDKBIN/aarch64-linux-android26-clang"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$NDKBIN/armv7a-linux-androideabi26-clang"
cargo build --release --target aarch64-linux-android --target armv7-linux-androideabi
# outputs:
#   target/aarch64-linux-android/release/rapidtally-ace
#   target/aarch64-linux-android/release/librapidtally_format.so
#   target/armv7-linux-androideabi/release/... (same names)
```

(Requires the Android NDK; `ANDROID_NDK_HOME` must point at it.)

## Quick on-device check (Termux)

```bash
# push the zip, extract, then:
chmod +x rapidtally-ace
./rapidtally-ace --version
./rapidtally-ace inspect /sdcard/Download/your-file.qbw
```

# Bundled bindings

These bindings were generated with bindgen 0.73.2 from the bundled FFmpeg 9.0.2
headers, including avcodec, avdevice, avfilter, avformat, swresample, and swscale.
The nine Rust files remain in Git for review. The published crate contains
`bindings.tar.xz`, and its build script extracts the requested target's file.
`manifest.json` records the source archive, bindings archive, and individual
binding files' SHA-256 hashes.

Generate the native bindings and a complete bundled header installation with:

```sh
cargo build --features generate-bindings
```

Copy the resulting `OUT_DIR/bindings.rs` to the corresponding target file here.
The generated `OUT_DIR/dist` provides the exact headers for the other targets.
For example, generate the ARM64 iOS simulator bindings with:

```sh
FFMPEG_DIR=/path/to/bundled/dist \
BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --sdk iphonesimulator --show-sdk-path) -mios-simulator-version-min=14.0" \
cargo check --no-default-features \
  --features generate-bindings,avcodec,avdevice,avfilter,avformat,swresample,swscale \
  --target aarch64-apple-ios-sim
```

Use the corresponding macOS, iPhoneOS, or iPhoneSimulator SDK for each target,
and match the deployment version reported by
`rustc --target <target> --print deployment-target`. Generate all target
files before rebuilding the archive with those files at its root and updating
the hashes.

Enums carry conditional `non-exhaustive-enums` attributes, so the same files
support that feature both enabled and disabled.

Linux bindings are bundled for `x86_64-unknown-linux-gnu` and
`aarch64-unknown-linux-gnu`. Generate them on the matching Linux architecture with
`cargo build --features generate-bindings`, or cross-compile from macOS with
`cargo zigbuild --target <target> --features generate-bindings` and libclang available.
The build honors cc-rs compiler/archiver settings, including cargo-zigbuild wrappers.
When cross-compiling, set `NM` to a target-capable `nm` such as LLVM's `llvm-nm`.

The bundled Linux configuration checks for both the legacy sysctl function and its
header. This avoids enabling an unusable API when targeting older glibc symbol sets
with newer Linux headers. Ordinary builds extract these bindings and do not run bindgen.

Rebuild `bindings.tar.xz` with all nine Rust files at its root, using deterministic
metadata (zero uid/gid/mtime and empty owner names), and update `manifest.json` hashes.

Android bindings cover ARM64 and x86_64 and were generated using NDK 29.0.14033849
at API 28 with bindgen 0.73.2. Use `cargo ndk -t arm64-v8a --platform 28 build --features
generate-bindings` (or `-t x86_64`) with libclang installed. The generator must see
the NDK sysroot, target-specific include directory and the same Android API level
as the C compiler. Repack all nine targets and update every archive/file hash.

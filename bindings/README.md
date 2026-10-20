# Apple bindings

These bindings were generated with bindgen 0.73.2 from the bundled FFmpeg 9.0.2
headers, including avcodec, avdevice, avfilter, avformat, swresample, and swscale.
The five Rust files remain in Git for review. The published crate contains
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
`rustc --target <target> --print deployment-target`. Generate all five target
files before rebuilding the archive with those files at its root and updating
the hashes.

Enums carry conditional `non-exhaustive-enums` attributes, so the same files
support that feature both enabled and disabled.

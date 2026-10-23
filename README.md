[![ramp-ffmpeg-sys on crates.io](https://img.shields.io/crates/v/ramp-ffmpeg-sys)](https://crates.io/crates/ramp-ffmpeg-sys)

RAMP Stack's fork of [ffmpeg-sys-next](https://github.com/zmwangx/rust-ffmpeg-sys),
originally derived from [ffmpeg-sys](https://github.com/meh/rust-ffmpeg-sys).
It includes the Apple framework-linking fix, bundled FFmpeg 9.0.2 C source, and
pregenerated Apple, Linux, and Android bindings. The safe wrapper is
[ramp-ffmpeg](https://github.com/ramp-stack/ffmpeg).

## Building

The default features compile the bundled C source into static libraries with portable
CPU settings. macOS, iOS, and Linux GNU builds use pregenerated bindings, so no installed FFmpeg,
pkg-config, libclang, Git, or build-time source download is required. A native C
compiler, Make, and the target SDK are still required. Intel targets automatically build
bundled NASM to preserve optimized assembly; no separate NASM installation is needed.
The NASM dependency is enabled by `bundled-nasm` (included in `build-portable`) and is
compiled only for x86/x86_64 targets. ARM targets skip it. When cross-compiling to Intel,
NASM itself is built for the host machine and used only during the build.
On Apple platforms, install Xcode's command-line tools and the SDK for the target.

The pregenerated bindings cover Apple Silicon and Intel macOS, arm64 iOS devices,
arm64/x86_64 iOS simulators, x86_64/aarch64 Linux GNU, and ARM64/x86_64 Android targets. They are used only with the bundled FFmpeg source.
Other targets and system/prebuilt FFmpeg installations require the explicit
`generate-bindings` feature and libclang. To use an installed FFmpeg, disable default
features and enable `generate-bindings` plus the required component features.

The bundled source and its provenance are described in [vendor/README.md](vendor/README.md).
FFmpeg retains its own licensing; see [LICENSE.md](LICENSE.md).

A word on versioning: major and minor versions track major and minor versions of FFmpeg, e.g. 4.2.x of this crate has been updated to support the 4.2.x series of FFmpeg. Patch level is reserved for bug fixes of this crate and does not track FFmpeg patch versions.

## Feature flags

In addition to feature flags declared in `Cargo.toml`, this crate performs various compile-time version and feature detections and exposes the results in additional flags. These flags are briefly documented below; run `cargo build -vv` to view more details.

- `ffmpeg_<x>_<y>` flags (new in v4.3.2), e.g. `ffmpeg_4_4`, indicating the FFmpeg installation being compiled against is at least version `<x>.<y>`. Currently available:

  - `ffmpeg_3_0`
  - `ffmpeg_3_1`
  - `ffmpeg_3_2`
  - `ffmpeg_3_3`
  - `ffmpeg_3_4`
  - `ffmpeg_4_0`
  - `ffmpeg_4_1`
  - `ffmpeg_4_2`
  - `ffmpeg_4_3`
  - `ffmpeg_4_4`
  - `ffmpeg_5_0`
  - `ffmpeg_5_1`
  - `ffmpeg_6_0`
  - `ffmpeg_6_1`
  - `ffmpeg_7_0`
  - `ffmpeg_7_1`
  - `ffmpeg_8_0`
  - `ffmpeg_8_1`
  - `ffmpeg_9_0`

- `avcodec_version_greater_than_<x>_<y>` (new in v4.3.2), e.g., `avcodec_version_greater_than_58_90`. The name should be self-explanatory.

- `ff_api_<feature>`, e.g. `ff_api_vaapi`, corresponding to whether their respective uppercase deprecation guards evaluate to true.

- `ff_api_<feature>_is_defined`, e.g. `ff_api_vappi_is_defined`, similar to above except these are enabled as long as the corresponding deprecation guards are defined.

## Android

Install the Android NDK, the Rust Android target, and `cargo-ndk`. The Android bundle
supports `aarch64-linux-android` and `x86_64-linux-android`, with API 28 as the
validated minimum. For example:

```sh
cargo ndk -t arm64-v8a --platform 28 build
```

The build uses the NDK compiler, archiver, ranlib, nm, strip and sysroot. Preserve
tool symlink names: LLVM selects ranlib/strip behavior from the invoked executable
name. Default builds use the bundled bindings and do not need libclang. The final
application linker must use 16 KB load-segment alignment and the APK packager must
align uncompressed native libraries for 16 KB pages.

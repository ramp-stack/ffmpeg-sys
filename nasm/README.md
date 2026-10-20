# ramp-nasm

Builds the bundled NASM 3.02 release source for the build host and exposes its
executable to dependent build scripts as `DEP_RAMP_NASM_BIN`. There is no Rust
runtime API or system installation.

`ramp-ffmpeg-sys` enables this package only for Intel targets when its
`bundled-nasm` feature is enabled. NASM runs on the host even when the requested
FFmpeg target uses a different architecture or an iOS SDK.

The build uses a C compiler, Make, and standard Unix shell tools. Apple hosts use
Xcode's macOS compiler and SDK. Perl, autoconf, an installed NASM, and build-time
network access are unnecessary. The included zlib is built with NASM, avoiding an
external compression-library dependency.

The exact archive and checksums are documented in [vendor/README.md](vendor/README.md).
NASM uses the [BSD 2-Clause license](LICENSE); its included zlib uses the
[Zlib license](ZLIB-LICENSE). All original sources and notices remain in the archive.

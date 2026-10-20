# Component licenses

The Rust binding code retains the upstream [WTFPL license](LICENSE.WTFPL).

The bundled FFmpeg source retains its original licenses. Its default native-library
configuration is LGPL version 2.1 or later. Optional GPL components and files with
other licenses are also included in the source archive; they retain their individual
license notices. See [FFmpeg's license overview](vendor/FFMPEG-LICENSE.md) and the
`COPYING.*` files inside `vendor/ffmpeg-9.0.2.tar.xz`.

The crate's default build does not enable GPL, version-3-only, or nonfree features.
Enabling the corresponding build features changes the resulting FFmpeg library's
license as described by FFmpeg.

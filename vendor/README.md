# FFmpeg source

`ffmpeg-9.0.2.tar.xz` contains FFmpeg tag `n9.0.2`, commit
`946fcce07b6dcd0331c8cc609192aeff5e1924f8`, from
https://github.com/FFmpeg/FFmpeg.

SHA-256: `b8663330d81917929710c9d6e644fbc0004a6c491b71900c1605a470a46e39b7`.

The archive preserves all C, header, assembly, build, and license files. Only
`tests/ref/` reference outputs and `doc/*.texi` manuals are omitted to fit the
registry package limit. The crate disables documentation and test builds.
The source files are otherwise unchanged.

The build script extracts this archive using Rust libraries and compiles it with
FFmpeg's configure/Make build. It does not download source or invoke Git.
`FFMPEG-LICENSE.md` is copied from the archive's `LICENSE.md`; the complete
`COPYING.*` license texts remain inside the source archive.

# NASM source

`nasm-3.02.tar.xz` is the unmodified official NASM 3.02 release archive:

https://www.nasm.us/pub/nasm/releasebuilds/3.02/nasm-3.02.tar.xz

Size: 1,531,636 bytes.

SHA-256: `87336eba53b4acfe917424ab5d500d2b0054d9f5148d35c2273ccf2cfb712f0d`.

The release archive includes generated source files, so building it does not
require Perl or autoconf. Extraction preserves file timestamps to avoid
regenerating those files. The build runs `sh configure --without-zlib` and
`make nasm`; `--without-zlib` selects the zlib sources included in the archive.
No source files are modified, downloaded, or installed outside Cargo's output directory.

`../LICENSE` is copied from NASM's `LICENSE`. `../ZLIB-LICENSE` contains the copyright
notice from the included `zlib/README`. The complete original notices remain in
the archive.

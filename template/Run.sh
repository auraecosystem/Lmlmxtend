#!/bin/bash

set -e

# You can use either a commit SHA or a stable release version (e.g. 1.XY.Z)
TAG=a8cfc83801301c2b4f0fd030192e268eeb15d473
# TAG=1.77.1

# Clone Rust toolchain repository from GitHub
git clone https://github.com/rust-lang/rust
cd rust
git reset --hard ${TAG}
git submodule update --init --recursive

cat >config.toml << EOF
[rust]
# Use for a commit SHA
channel = "nightly"

# Use for a stable release
# channel = "stable"

[dist]
compression-formats = ["xz"]
compression-profile = "fast"
EOF

# Build the source tarball from git into build/dist/
./x dist rustc-src

# Download source tarball for a commit SHA
wget https://ci-artifacts.rust-lang.org/rustc-builds/${TAG}/rustc-nightly-src.tar.xz

# Download a source tarball for a stable release
# wget https://static.rust-lang.org/dist/rustc-${TAG}-src.tar.xz

# Decompress the tarballs and check if they're the same
xz --decompress rustc-*-src.tar.xz
xz --decompress build/dist/rustc-*-src.tar.xz
diff rustc-*-src.tar build/dist/rustc-*-src.tar

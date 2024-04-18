#!/bin/bash
build=$(date +%FT%T%z)
linux="target/x86_64-unknown-linux-musl/release/gitql"
if [ "$1" = "all" ]; then
  build=$build cargo build --release --all-features --all-targets
elif [ "$1" = "offline" ]; then
  build=$build cargo build --release --all-features --all-targets --offline
elif [ "$1" = "check" ]; then
  build=$build cargo check --release --all-features --all-targets
else
  build=$build cargo build --release --all-features --all-targets
fi
if [ -f $linux ]; then upx $linux; fi

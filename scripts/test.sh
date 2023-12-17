#!/bin/bash

build=$(date +%FT%T%z)

list="gitql-ast gitql-cli gitql-engine gitql-parser"
for item in $list; do
  pushd crates/"$item" || exit
  # rustup update --no-self-update stable
  build=$build cargo test --all-features --all-targets -- --nocapture
  popd || exit
done

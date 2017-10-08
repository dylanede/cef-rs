#!/bin/sh

cargo build --release
cp -r $CEF_DIST_ROOT/Release/* target/release
cp -r $CEF_DIST_ROOT/Resources/* target/release


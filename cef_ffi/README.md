This crate contains a build.rs script to generate FFI bindings at compile time
for a local copy of CEF, Chromium Embedded Framework.

This crate does NOT provide a high-level Rust API, but should be a good layer to build a high-level API on top of.

The environment variable CEF_DIST_ROOT is required to point to an extracted CEF
distribution archive.

The example in examples/hello_cef contains a helper script make_macos_app.sh
that helps create the reqired app sctructure and an example of the required
install_name_tool usage.

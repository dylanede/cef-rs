This crate contains a build.rs script to generate FFI bindings at compile time
for a local copy of CEF, Chromium Embedded Framework.

This crate does NOT provide a high-level Rust API, but should be a good layer to build a high-level API on top of.

The environment variable CEF_DIST_ROOT is required to point to an extracted CEF
distribution archive, at the time of writing the expected structure is something like:

	Release/
	cmake/
	include/
	libcef_dll/
	CMakeLists.txt
	LICENSE.txt
	README.txt

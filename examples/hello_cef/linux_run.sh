# TODO: Investigate what the recommended approach is for bundled .so libs.
pushd ./target/release
LD_LIBRARY_PATH=. ./hello_cef
popd


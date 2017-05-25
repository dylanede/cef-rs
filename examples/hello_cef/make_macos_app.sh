#!/bin/sh

cargo build --release

set name=hello_cef
rm -rf output/hello_cef.app
mkdir -p output/hello_cef.app/Contents/Frameworks
mkdir -p output/hello_cef.app/Contents/Frameworks/hello_cef\ Helper.app/Contents/MacOS
# need to provide a helper executable, can it be the same?
touch    output/hello_cef.app/Contents/Frameworks/hello_cef\ Helper.app/Contents/Info.plist
touch    output/hello_cef.app/Contents/Frameworks/hello_cef\ Helper.app/Contents/Pkginfo
mkdir -p output/hello_cef.app/Contents/MacOS
mkdir -p output/hello_cef.app/Contents/Resources
cp resources/Info.plist output/hello_cef.app/Contents/Info.plist
cp resources/grump.icns output/hello_cef.app/Contents/Resources/
touch    output/hello_cef.app/Contents/Pkginfo
cp -R $CEF_DIST_ROOT/Release/* output/hello_cef.app/Contents/Frameworks/
cp target/release/hello_cef output/hello_cef.app/Contents/MacOS/
pushd output/hello_cef.app/Contents/MacOS
install_name_tool -change @rpath/Frameworks/Chromium\ Embedded\ Framework.framework/Chromium\ Embedded\ Framework @executable_path/../Frameworks/Chromium\ Embedded\ Framework.framework/Chromium\ Embedded\ Framework hello_cef
popd

#open ./output
./output/hello_cef.app/Contents/MacOS/hello_cef

#!/bin/sh

cargo build --release

name=hello_cef
rm -rf output/${name}.app
mkdir -p output/${name}.app/Contents/Frameworks
mkdir -p output/${name}.app/Contents/Frameworks/${name}\ Helper.app/Contents/MacOS
# need to provide a helper executable, can it be the same?
cp resources/Info.plist output/${name}.app/Contents/Frameworks/${name}\ Helper.app/Contents/Info.plist
touch    output/${name}.app/Contents/Frameworks/${name}\ Helper.app/Contents/Pkginfo
mkdir -p output/${name}.app/Contents/MacOS
mkdir -p output/${name}.app/Contents/Resources
cp resources/Info.plist output/${name}.app/Contents/Info.plist
cp resources/grump.icns output/${name}.app/Contents/Resources/
touch    output/${name}.app/Contents/Pkginfo
cp -R $CEF_DIST_ROOT/Release/* output/${name}.app/Contents/Frameworks/
cp target/release/${name} output/${name}.app/Contents/MacOS/
pushd output/${name}.app/Contents/MacOS
install_name_tool -change @rpath/Frameworks/Chromium\ Embedded\ Framework.framework/Chromium\ Embedded\ Framework @executable_path/../Frameworks/Chromium\ Embedded\ Framework.framework/Chromium\ Embedded\ Framework ${name}
popd
cp target/release/${name} output/${name}.app/Contents/Frameworks/${name}\ Helper.app/Contents/MacOS/${name}\ Helper
pushd output/${name}.app/Contents/Frameworks/${name}\ Helper.app/Contents/MacOS
install_name_tool -change @rpath/Frameworks/Chromium\ Embedded\ Framework.framework/Chromium\ Embedded\ Framework @executable_path/../../../Chromium\ Embedded\ Framework.framework/Chromium\ Embedded\ Framework ${name}\ Helper
popd

#open ./output
#./output/${name}.app/Contents/MacOS/${name}


# Usage
# package_macos.sh $VERSION

TARGET_PATH=./target/macos/Tick.app
CONTENT_PATH=$TARGET_PATH/Contents
BIN_PATH=$CONTENT_PATH/MacOS
RESOURCE_PATH=$CONTENT_PATH/Resources
VERSION=$1

# Make directories
rm -rf $TARGET_PATH
mkdir -p $BIN_PATH
mkdir -p $RESOURCE_PATH

printf '<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
        <key>CFBundleDevelopmentRegion</key>
        <string>English</string>
        <key>CFBundleExecutable</key>
        <string>Tick</string>
        <key>CFBundleIconFile</key>
        <string>Tick.icns</string>
        <key>CFBundleIdentifier</key>
        <string>io.mtso.tick</string>
        <key>CFBundleInfoDictionaryVersion</key>
        <string>6.0</string>
        <key>CFBundleName</key>
        <string>Tick</string>
        <key>CFBundlePackageType</key>
        <string>APPL</string>
        <key>CFBundleShortVersionString</key>
        <string>%s</string>
        <key>CFBundleSignature</key>
        <string>xmmd</string>
        <key>CFBundleVersion</key>
        <string>%s</string>
        <key>NSAppleScriptEnabled</key>
        <string>NO</string>
</dict>
</plist>' $VERSION $VERSION > $CONTENT_PATH/Info.plist

cp ./target/release/Tick $BIN_PATH/Tick
cp ./assets/Tick.icns $RESOURCE_PATH/Tick.icns

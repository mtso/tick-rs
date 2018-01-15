# Tick

Experimental Rust app that uses bindings to GTK.

## Requirements

- GTK+ 3 `brew install gtk+3`

## Build MacOS App

```sh
cargo build -q && ./bin/package_macos.sh $VERSION
```

Where `$VERSION` is the version string to put in the app bundle's `Info.plist` file. The `--quiet` option is used to suppress the `snake_case` warning for package names.

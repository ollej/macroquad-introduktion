# Build for iOS

You can build your Macroquad game to run on iPhone mobile phones and iPads.

```admonish info title="More information"
More detailed information on how to build for iOS is available in the article
[Macroquad on iOS](https://macroquad.rs/articles/ios/) on the Macroquad homepage.
There you can find information on how to access logs, building for real
devices, and signing your app.
```

## Create a directory

An iOS app is a regular directory with the file extension `.app`.

```sh
mkdir MyGame.app
```

For our game, the directory structure in the `MyGame.app` directory is the same
as when we run the game with `cargo run` from the root of the crate. The
binary file and `assets` directory should be placed next to each other. You
also need an `Info.plist` file.

Start by adding the `assets`.

```sh
cp -r assets MyGame.app
```

## Build the binary

You need to add the Rust target for iOS. For the simulator you should use
Intel binaries and for the real devices you should use ARM binaries. This
guide will only cover how to try the game in the simulator. How to try the
game on a real device is covered in the [Macroquad on
iOS](https://macroquad.rs/articles/ios/) article on the Macroquad homepage.

```sh
rustup target add x86_64-apple-ios
```

After this you can build an executable binary for the iOS Simulator using the
following command:

```sh
cargo build --release --target x86_64-apple-ios
```

## Copy the binary file

Copy the executable binary file to the game directory.

```sh
cp target/x86_64-apple-ios/release/my-game MyGame.app
```

## Create Info.plist

Create a text file for the app metadata with the name `Info.plist` in the
`MyGame.app` directory with the following content:

```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>CFBundleExecutable</key>
<string>my-game</string>
<key>CFBundleIdentifier</key>
<string>com.mygame</string>
<key>CFBundleName</key>
<string>mygame</string>
<key>CFBundleVersion</key>
<string>1</string>
<key>CFBundleShortVersionString</key>
<string>1.0</string>
</dict>
</plist>
```

## Setup the simulator

For this step you need to have XCode and at least one simulator image
installed. You'll find XCode in the App Store app. You can add simulators via
the command line or via XCode. In version 15.1 of XCode you can do it via
`Settings...` &rarr; `Platforms` and then choose between the available iOS
versions. There is also a button (`+`) to add more iOS versions.

To add simulators via the command line you first need to run the command
`xcrun simctl list` to get a list of all the available simulators. Copy the
hex code for the simulator you want and use it as argument to the `xcrun simctl
boot` command. You only need to do this the first time you run the simulator.

```bash
xcrun simctl list
xcrun simctl boot <hex string>
```

## Run the simulator

The command we'll use to install and run the game, `xcrun simctl`, chooses a
simulator with the argument `booted`. This means that you first need to start
a simulator and to make things predictable, you should only run one simulator
at a time. This can also be done using the terminal, but the easiest way is to
start the Simulator app and then start the simulator you want via `File`
&rarr; `Open Simulator`.

To start the simulator using the terminal, use the following command:

```sh
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app/
```

## Install the game

You can install the game by dragging the directory `MyGame.app` and dropping
it on the running simulator. But since you probably want to reinstall it
multiple times it is more efficient to use the terminal with this command:

```sh
xcrun simctl install booted MyGame.app/
```

## Start the game

This can be done using the running simulator or via the terminal. In our
`Info.plist` file we specified `CFBundleIdentifier` as `com.mygame`, which we
will use to start the game.

```sh
xcrun simctl launch booted com.mygame
```

```admonish note title="Please note!"
You'll notice that the game isn't fully adapted to be run on a mobile platform yet.
To start with you can read about the function `touches()` in the
[Macroquad documentation](https://docs.rs/macroquad/latest/macroquad/input/index.html)
or more information about how touch interfaces work.
```

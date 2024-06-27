# Build your game for desktop platforms

Macroquad supports multiple desktop platforms, such as Windows, MacOS, and
Linux. It's possible to cross-compile for other platforms than the one you are
using. But it might need other tools that are not described in this guide.
It's easiest to use a build system that has support for different platforms.

## Build for Windows

If you want to build your game to be run on Windows you need to install a Rust
build target. Both the MSVC and GNU build targets are supported.

### Build using Windows GNU target

Before running the build the for first time you need to install the build
target. You will only have to run this command once.

```sh
rustup target add x86_64-pc-windows-gnu
```

To build the game, use the following command:

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

The binary file created will be stored in the directory
`target/x86_64-pc-windows-gnu/release/`.

### Build using Windows MSVC target

Before running the build the for first time you need to install the build
target. You will only have to run this command once.

```sh
rustup target add x86_64-pc-windows-msvc
``` 

To build the game, use the following command:

```sh
cargo build --release --target x86_64-pc-windows-msvc
```

The binary file created will be stored in the directory
`target/x86_64-pc-windows-msvc/release/`.

## Build for Linux

To build your game with Macroquad on Linux you will need a couple of
development packages. Below are a few instructions how to install these
packages on some common Linux distributions.

### Install packages

#### Ubuntu

These system packages must be installed to build on Ubuntu.

```sh
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

#### Fedora

These system packages must be installed to build on Fedora.

```sh
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel
```

#### Arch Linux

These system packages must be installed to build on Arch Linux.

```sh
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

### Build using Linux GNU target

Before running the build for the first time you need to install the build
target. You will only have to run this command once.

```sh
rustup target add x86_64-unknown-linux-gnu
```

To build the game, use the following command:

```sh
cargo build --release --target x86_64-unknown-linux-gnu
```

The binary file created will be stored in the directory
`target/x86_64-unknown-linux-gnu/release/`.

## Build using MacOS

To build on MacOS there are two possible targets: `x86_64-apple-darwin` is
used for older Intel-based Mac computers, and `aarch64-apple-darwin` build is
used for newer Apple Silicon-based Mac computers.

### Build using x86-64 Apple Darwin target

Before running the build for the first time you need to install the build
target. You will only have to run this command once.

```sh
rustup target add x86_64-apple-darwin
```

To build the game, use the following command:

```sh
cargo build --release --target x86_64-apple-darwin
```

The binary file created will be stored in the directory
`target/x86_64-apple-darwin/release/`.

### Build using aarch64 Apple Darwin target

Before running the build for the first time you need to install the build
target. You will only have to run this command once.

```sh
rustup target add aarch64-apple-darwin
```

To build the game, use the following command:

```sh
cargo build --release --target aarch64-apple-darwin
```

The binary file created will be stored in the directory
`target/aarch64-apple-darwin/release/`.

## Package the game

To share your game with others you need to package the game binary file
together with all the assets needed to run the game. Here are a couple of
examples on how to do this using a terminal.

### Windows

```sh
cp target/x86_64-pc-windows-gnu/release/my-game.exe ./
tar -c -a -f my-game-win.zip my-game.exe assets/*
```

### Linux

```sh
cp target/x86_64-pc-linux-gnu/release/my-game ./
tar -zcf my-game-linux.zip my-game assets/*
```

### Mac

```sh
cp target/aarch64-apple-darwin/release/my-game ./
zip -r my-game-mac.zip my-game assets/*
```


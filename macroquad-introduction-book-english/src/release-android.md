# Build for Android phones

Using Macroquad it's possible to build your game to be run on Android phones.
We will build an APK file that can be installed on Android phones or added to
the Google Play store. We'll describe how to build the game using
[Docker](https://www.docker.com/get-started/), so you need to have that
installed to continue.

Since mobile platforms don't have physical keyboards you will also have to add
support for controlling the game using touch controls.

```admonish note
Read about the function `touches()` in the [Macroquad
documentation](https://docs.rs/macroquad/latest/macroquad/input/index.html)
for more information on how touch controls work.
```

## Install the Docker image

Before you build an APK file for Android you need to pull the Docker image
`notfl3/cargo-apk`.

```sh
docker pull notfl3/cargo-apk
```

## Build the APK file

Using this command you can build an APK file. It will take quite some time
since it will do three full builds, one for each Android target.

```sh
docker run 
  --rm 
  -v $(pwd):/root/src 
  -w /root/src 
  notfl3/cargo-apk cargo quad-apk build --release
```

After this you will have an APK file in the directory
`target/android-artifacts/release/apk`.

## Configuration

To ensure that Android can find all the assets, you need to add some
configuration to the `Cargo.toml` file to define where the assets can be
found.

```toml
[package.metadata.android]
assets = "assets/"
```

```admonish info
On the Macroquad homepage there are more detailed instructions on how to 
[build for Android](https://macroquad.rs/articles/android/). It has tips on
how to speed up the build, how to build manually without Docker, and how to
sign the APK file which is needed to upload it to the Google Play Store.
```

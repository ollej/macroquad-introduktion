# Publish your game on the web

Since you can compile a Macroquad game to WebAssembly it's possible to run the
game in a web browser. These are instructions on how to create a web page to
run your game. This web page can be published on a web account so that people
can play your game directly in the browser without having to download
anything.

## Install WASM build target

Start by installing the build target for WebAssembly using the command
`rustup`.

```sh
rustup target add wasm32-unknown-unknown
```

## Build a WebAssembly binary

Using the WebAssembly target you can build a WASM binary file that can be
loaded from a web page.

```sh
cargo build --release --target wasm32-unknown-unknown
```

The WASM binary file will be placed in the directory
`target/wasm32-unknown-unknown/release/` with the extension `.wasm`.

## Copy WebAssembly binary

You need to copy the WebAssembly binary to the root of your crate, in the same
place that the `assets` directory is placed.

If you have named your crate something else than `my-game` the name of the
binary will have the same name, but with the file extension `.wasm`.

```sh
cp target/wasm32-unknown-unknown/release/my-game.wasm .
```

## Create an HTML page

You will need an HTML page to load the WebAssembly binary. It needs to load a
javascript file from Macroquad which contains code to run the WebAssembly
binary and communicate with the browser. You also need to add a canvas element
that Macroquad will use to draw the graphics. Remember to change the name of
the WebAssembly binary file in the `load()` call from `my-game.wasm` to the
name of your game if you have changed it.

Create a file with the name `index.html` in the root of your crate with the
following content:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>My Game</title>
    <style>
        html,
        body,
        canvas {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>
<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/macroquad/blob/master/js/mq_js_bundle.js -->
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script>load("my-game.wasm");</script> <!-- Your compile WASM binary -->
</body>
</html>
```

## Test the game in a browser

You should be able to start a web server and open the page in a web browser.

### Install a simple web server

To serve your game locally on your computer you can install a simple web
server with the following command. This is only to be able to test the game
locally before you upload it to a proper web account.

```sh
cargo install basic-http-server
```

### Run the web server

This command will start the web server and print an address where you can
reach the web page. Open your web browser and load the URL, this will be
something similar to `http://localhost:4000`. The game should now run in your
browser instead of as a native application.

```sh
basic-http-server .
```

### Publish your game

If you have access to a web account you can publish the files there to let
other people play your game. You need to upload the HTML file, the WASM file,
and the `assets` directory.

```
index.html
my-game.wasm
assets/*
```

```admonish note
This is a reminder that there are instructions at the end of
[chapter 1](ch1-first-program.md#publish-on-the-web-if-you-want)
with instructions on how to automatically publish the game on GitHub without
using a web account. In that case you need to use the updated `deploy.yml`
from [chapter 10 – Graphics](ch11-graphics.md#update-web-publishing).
```

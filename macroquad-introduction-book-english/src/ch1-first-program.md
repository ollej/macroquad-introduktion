# Your first Macroquad app

Now it's time to develop your first application with Macroquad. Start by
installing the programming language Rust if you don't already have it.

![Screenshot](images/first-program.png#center)

## Implementation

Create a new Rust project using the Cargo command line tool and add
`macroquad` with version `0.4` as a dependency. If you want, you can give your
game a more interesting name than "my-game".

```sh
cargo new --bin my-game
cd my-game/
cargo add macroquad@0.4
```

Your `Cargo.toml` file should now look like this:

```toml
[package]
name = "my-game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
macroquad = "0.4"
```

Open the file `src/main.rs` in your favorite text editor and change the
content to look like this:

```rust
{{#include ../../my-game/examples/first-program.rs}}
```

Run your application with `cargo run` and a new window with a dark purple
background will open once the compilation has finished.

## Description of the application

The first line is used to import everything needed to from Macroquad. This is
easiest to do by importing `macroquad::prelude::*`, but it is also possible to
import only the features that are used.

The attribute `#[macroquad::main("My game")]` is used to tell Macroquad which
function that will be run when the application starts. When the application is
started, a window will be opened with the argument as the title, and the
function will be executed asynchronously. If you have named your game
something more interesting you should change the text `My game´ to the name of
your game.

```admonish info
To change the configuration for the window, like the size or if it should
start in fullscreen mode you can use the struct 
[Conf](https://docs.rs/macroquad/latest/macroquad/window/struct.Conf.html)
instead of the string as an argument.
```

Inside the `main` function there is a loop that never ends. All the game logic
will be placed inside this game loop and will be executed every frame. In our
case we clear the background of the window with a dark purple color with the
function `clear_background(DARKPURPLE)`. At the end of the loop is the
function `next_frame().await` which will wait until the next frame is
available.

```admonish note
Even if `clear_background()` isn't used explicitly, the screen will be cleared
with a black color at the start of each frame.
```

```admonish tip title="Challenge" class="challenge"
Try to change the background of the window to your favorite color.
```

## Quiz

Try your new knowledge by answering this quiz before you continue to the next
chapter.

{{#quiz ../quizzes/first-program.toml}}

## Publish on the web (if you want)

One of the big advantages with Rust and Macroquad is that it is very easy to
compile a standalone application for different platforms. How this works will
be explained in a [later chapter](release-game.md) of this guide. If you want,
you can setup a GitHub deploy action to publish a web version of the game
every time you commit.

When you created the game with `cargo new` a local Git repository was also
created. Start by committing your changes locally. After that you can create a
repository on GitHub and push the code there.

```admonish note
The two files below refer to `my-game.wasm`. If you've changed the name of
your crate to something other than `my-game` you need to change those
references.
```

You need an HTML file to show the game. Create a file called `index.html` in
the root of the project/crate and add the following content:

```html
{{#include game.html}}
```

The following GitHub Actions Workflow will compile the game to WASM and put
all files in place so that the game will work on the web. Place the code in 
`.github/workflows/deploy.yml`.

```yaml
{{#include ../../my-game/examples/deploy-early.yml}}
```

Commit and push! You can follow the build under the **Actions** page of the
repository. The first time you push your code the game will be built and all
files placed in the correct place, in the root of the branch `gh-pages`, but
no web page will be created. You need to change a configuration of the GitHub
repository under **Settings** > **Pages** > **Build and deployment**. Set
`gh-pages` as the branch to deploy the web page from.

![Github Pages Settings](images/github-pages-settings.png)

When the build is done you will be able to play your game on
`https://<your-github-account>.github.io/<repository-name>`.

It won't be much of game yet, only a purple background. But you have delivered
early and the project is configured for continuous delivery. Every time you
add functionality to the game and push the code to GitHub you will be able to
play the latest version of the game on the web. In the next chapter things
will start to move!

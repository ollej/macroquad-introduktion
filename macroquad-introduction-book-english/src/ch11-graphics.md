# Graphics

It's time to add some graphics to our game to make it look more like a real
game. We will do it in three steps so that there won't be too many changes at
once. To begin with we'll add code to load textures directly in our `main`
function and change the draw function in the game loop. In a later chapter we
will look at how to extract the texture loading into a separate function.

Before we make any code changes we need to download all the needed resources.
Download this [package with graphics and sound](assets.zip) and extract it to
a directory called `assets` in the root directory of your game.

All the resources are public domain and are primarily from the website
[OpenGameArt.org](https://opengameart.org/) where there are lots of different
resources to develop games.

The file structure for your game should look like this:

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── assets
│   ├── 8bit-spaceshooter.ogg
│   ├── atari_games.ttf
│   ├── button_background.png
│   ├── button_clicked_background.png
│   ├── enemy-big.png
│   ├── enemy-medium.png
│   ├── enemy-small.png
│   ├── explosion.png
│   ├── explosion.wav
│   ├── laser-bolts.png
│   ├── laser.wav
│   ├── ship.png
│   └── window_background.png
└── src
    ├── main.rs
    └── starfield-shader.glsl
```

## Update web publishing

If you chose to setup web publishing of your game to Github Pages in the 
[first chapter](ch1-first-program.md#publicera-på-webben-om-du-vill) you will
need to update the file `.github/workflows/deploy.yml` to make sure the assets
are included when publishing.

The `assets` directory needs to be created:

```yaml
{{#include ../../my-game/examples/deploy-with-assets.yml:assets-mkdir}}
```

The asset files need to be copied into the `assets` directory:

```yaml
{{#include ../../my-game/examples/deploy-with-assets.yml:assets-cp}}
```

The complete deploy configuration should now look like this:

```yaml
{{#include ../../my-game/examples/deploy-with-assets.yml:all}}
```

Commit your changes and push to GitHub and verify that the game still works
on:
* `https://<your-github-account>.github.io/<repository-name>`.

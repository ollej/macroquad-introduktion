# Spaceship and bullets

![Screenshot](images/screenshots-web/graphics-spaceship.gif#center)

To begin with we'll add graphics for the spaceship that the player controls.
It will be animated with two different sprites and will also have different
animations for when the spaceship moves to the left or right. We'll also add a
texture with animation for the bullets that the spaceship shoots.

## Implementation

### Import

The animation support in Macroquad is considered an experimental feature. It
might change in a future version of Macroquad. It is not included in the
prelude that we have imported, so we will have to import it explicitly.

Import the structs `AnimatedSprite` and `Animation` at the top of `main.rs`
file.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:import}}
```

### Configure assets directory

We need to start by defining where Macroquad should read the resources. We'll
use the function `set_pc_assets_folder()` that takes the path to the `assets`
directory relative to the root directory of the game. This is needed for
platforms that might place files in other places and also has the added
benefit that we don't need to add the directory name for every file we load.

Add the following code in the `main` function above the game loop:

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:assetsfolder}}
```

### Load textures

Load the image files used for the animation textures of the ship and bullets.
Use the function `load_texture()` to load a texture, which takes the name of
the file to load. This function is async, because it supports loading files
over HTTP in WebAssembly, so we need to call `await` to get the result.

Since loading files can fail, this function will return a `Result`. We will
call `expect()` on the result to stop the program if it wasn't possible to
load the file. This can happen if the file is missing, or it has wrong read
permissions. On WebAssembly it is possible that the HTTP request failed.

After loading the texture we'll set which kind of filter to use when scaling
the texture using the method `set_filter()`. We will use the filter
`FilterMode::Nearest` because we want to keep the pixelated look of the
sprites. This needs to be done on every texture that is loaded. For high
resolution textures it would be better to use `FilterMode::Linear` which gives
a linear scaling of the texture.

We'll load the file `ship.png` that contains the animations for the spaceship,
and the file `laser-bolts.png` that contains animations for two different
kinds of bullets.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:loadresources}}
```

```admonish info
The images are returned as the struct 
[`Texture2D`](https://docs.rs/macroquad/latest/macroquad/texture/struct.Texture2D.html)
that stores the image data in GPU memory. The corresponding struct for images
stored in CPU memory is 
[`Image`](https://docs.rs/macroquad/latest/macroquad/texture/struct.Image.html).
```

### Build a texture atlas

After loading all the textures we'll call the Macroquad function
`build_textures_atlas()` that will build an atlas containing all loaded
textures. This will ensure that all calls to `draw_texture()` and
`draw_texture_ex()` will use the texture from the atlas instead of each
separate texture, which is much more efficient. All textures need to be loaded
before this function is called.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:atlas}}
```

### Bullet animation

![Bullet spritesheet](assets/laser-bolts.png#pixelated)

The image `laser-bolts.png` is composed of four sprites, in two rows. These
make up the animations for two different types of bullets. We will name the
first one `bullet` and the second one `bolt`. Each animation is one row with
two frames each and they should be shown at 12 frames per second. The size of
the sprites is 16x16 pixels.

Each animation in a spritesheet is placed in a separate row, with the frames
next to each other horizontally. Each `Animation` should have a descriptive
name, define which row in the spritesheet it is, how many frames it has,
and how many frames should be displayed each second.

Create an `AnimatedSprite` with the `tile_width` and `tile_height` set to
`16`, and an array with an `Animation` struct for each of the two rows in the
spritesheet. The first one should be named `bullet` and have the row `0` and
the second one should have the name `bolt` and the row `1`. Both should have
`frames` set to `2` and `fps` set to `12`.

We will only use the second animation, so we'll use the method
`set_animation()` to define that we will be using the animation on row `1`.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:bulletsprite}}
```

### Spaceship animation

![Spaceship spritesheet](images/ship-spritesheet.png#right)

The spritesheet for the spaceship is in the image `ship.png` and we need to
define how the animations in the spritesheet should be displayed. We have to
create an `AnimatedSprite` for the ship as well. The size of each frame of the
spaceship spritesheet is 16x24 pixels, so we'll set `tile_width` to `16` and
`tile_height` to `24`. After that is an array with an `Animation` struct for
each animation in the spritesheet that we want to use.

There are five animations available in the spritesheet, with the first one in
the top row. We will only use three of the spritesheet animations in our
`AnimatedSprite`, the second and fourth row from the top in the spritesheet
are unused. The first one is used when flying up or down, so add an
`Animation` in the `AnimatedSprite` with `row` defined as `0` as the indexes
are 0-based, and the `name` set to `idle`. The ship will keep pointing up
regardless of if it moves up or down. The second `Animation` is for moving the
spaceship to the left, which will use the row with index `2` in the
spritesheet. Finally, the third `Animation` is used when moving the spaceship
to the right, and has the `row` index `4`. There are two `frames` in each
`Animation` and the `fps` should be set to 12 frames per second.

Finally we set `playing` to `true` so that the animation will be active.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:shipsprite}}
```

### Animate direction

For the spaceship we need to set which animation to use based on the direction
movement. In the code for moving the spaceship we will add a line where we use
the method `set_animation()` on the `ship_sprite`. We start by setting the
animation to `0` if it isn't turning in any direction, if it is moving to the
right we'll set the animation to `2`, and if it moves to the left we'll set
the animation to `1`. These numbers are indexes in the array of `Animation`
structs we defined in the `AnimatedSprite` for the spaceship, which means they
are 0-based.

```rust [hl,1,5,10]
{{#include ../../my-game/examples/graphics-spaceship.rs:updateship}}
```

### Change bullet size

Since the graphics for the bullets are larger than the tiny circle we used to
draw for them, we need to change the size and starting position when creating
a bullet.

```rust [hl,4,6]
{{#include ../../my-game/examples/graphics-spaceship.rs:shoot}}
```

### Update animations

In order for Macroquad to animate the textures, we need to call the method
`update()` on every sprite inside our game loop. Add the following two lines
below the code that updates the positions of enemies and bullets.

```rust [hl,8-9]
{{#include ../../my-game/examples/graphics-spaceship.rs:updatesprites}}
```

### Draw bullet animations

Now we can use the function `draw_texture_ex` to draw each frame of the
animation. Remove the lines that draw a circle for each bullet and insert
instead the code below. First we call the method `frame()` on the
`bullet_sprite` to get the current animation frame and set it to the variable
`bullet_frame`.

Inside the loop that draws all the bullets we'll call `draw_texture_ex` to
draw the bullet frame. It takes the `bullet_texture` as argument, and an `x`
and `y` position based on the size of the bullet. We also add the struct
`DrawTextureParams` with the fields `dest_size` and `source_rect`. The field
`dest_size` defines in which size the texture will be drawn, so we will use a
`Vec2` with the size of the bullet for both `x` and `y`. Finally we'll use
`bullet_frame.source_rect`, which is a reference to where in the texture the
current frame is placed.

```rust [hl,1,3-12]
{{#include ../../my-game/examples/graphics-spaceship.rs:drawbullets}}
```

```admonish info
By using
[`DrawTextureParams`](https://docs.rs/macroquad/0.3.25/macroquad/texture/struct.DrawTextureParams.html)
it is possible to change how the texture should be drawn. It is possible to
draw the texture rotated or mirrored with the fields `rotation`, `pivot`,
`flip_x`, and `flip_y`. 
```

### Draw the spaceship frames

Finally it's time to replace the circle with the texture for the spaceship. It
works in the same way as for the bullets. First we'll retrieve the current
frame from the animation sprite, and then we'll draw it using
`draw_texture_ex()`.

Because the spaceship animation isn't the same size in width and height, we'll
use `ship_frame.dest_size` to define which size should be drawn. To make it a
bit bigger we'll double the size.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:drawship}}
```

If everything works correctly, there should be animated graphics for both the
spaceship and the bullets when running the game.

## Improve loading times

Adding the following snippet at the end of the `Cargo.toml` file will ensure
that the assets are loaded much faster when running on a desktop computer.

```
[profile.dev.package.'*']
opt-level = 3
```

```admonish tip title="Challenge" class="challenge"
Try using the two extra spaceship animations to make the ship turn only
slightly just when it changes direction and then make it turn fully after a
short time.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/graphics-spaceship-and-bullets.toml}}

</div>

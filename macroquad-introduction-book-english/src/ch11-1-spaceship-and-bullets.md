# Spaceship and bullets

![Screenshot](images/graphics-spaceship.gif#center)

To begin with we'll add graphics for the spaceship that the player controls.
It will be animated with two different sprites, and will also have different
animations for when the spaceship moves to the left or right. We'll also add a
texture with animation for the bullets that the spaceship shoots.

## Implementation

### Import

The animation support in Macroquad is considered an experimental feature. It
might change in a future version of Macroquad. It's also not included in the
prelude that we have imported, so we will have to import it explicitly.

Import the structs `AnimatedSprite` and `Animation` at the top of `main.rs`
file.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:import}}
```

### Configure assets directory

We need to start by setting where Macroquad should read the resources. We'll
use the function `set_pc_assets_folder()` that takes the path to the `assets`
directory relative to the root directory of the game. This is needed for
platforms that might place files in other places, and we also have the added
benefit of not having to add the directory name for every file we load.

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
resolution textures it would be better to use `FilterMode::Linear` which makes
a linear scaling of the texture.

We'll load the files `ship.png` that contains the animations for the
spaceship, and `laser-bolts.png` that contains animations for two different
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
`build_textures_atlas` that will build an atlas containing all loaded
textures. This will ensure that all calls to `draw_texture` and
`draw_texture_ex` will use the texture from the atlas instead of each separate
texture which is much more efficient. All textures needs to be loaded before
this function is called.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:atlas}}
```

### Spaceship animation

![Spaceship spritesheet](assets/ship.png#pixelated)

We need to define how the animations in the textures should be displayed.
We'll do this by creating an `AnimatedSprite` for each texture. The size of
each frame of the spaceship texture is 16x24 pixels, so we'll set `tile_width`
to `16` and `tile_height` to `24`.

After that is an array describing all the animations included in the texture.
Each animation in a texture is placed in a separate row, with the frames next
to each other horizontally. Each `Animation` should have a descriptive name,
which row in the texture it is, how many frames it has, and how many frames to
display each second.

The spaceship has three animations, the first one is used when flying up or
down, the second is for moving left and the third is for moving right. There
are two frames per animation and they should be shown at 12 frames per
second. There are two more animations in the texture, on row 1 and 3 the ship
is shown slightly less angled.

Finally we set `playing` to `true` so that the animation will be active.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:shipsprite}}
```

### Bullet animation

![Bullet spritesheet](assets/laser-bolts.png#pixelated)

The bullet animation is very similar, it has two animations with two frames
each that should be shown at 12 frames per second. The size of the frames are
16x16 pixels. We will only use the second animation, so we'll use the method
`set_animation()` to define that we will be using the animation on row `1`.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:bulletsprite}}
```

### Animate direction

For the spaceship we need to set which animation to use based on the direction
it is moving. In the code for moving the spaceship we will add a line where we
use the method `set_animation()` on the `ship_sprite`. We start by setting the
animation to `0` if it isn't turning in any direction, if it is moving to the
right we'll set the animation to `2`, and if it moves to the left we'll set
the animation to `1`.

```rust [hl,1,5,10]
{{#include ../../my-game/examples/graphics-spaceship.rs:updateship}}
```

### Change bullet size

Since the graphics for the bullets is larger than the tiny circle we used to
draw we need to change the size and starting position when creating a bullet.

```rust [hl,4,6]
{{#include ../../my-game/examples/graphics-spaceship.rs:shoot}}
```

### Update animations

In order for Macroquad to animate the textures we need to call the method
`update()` on every sprite inside our game loop. Add the following two lines
below the code that updates the positions of enemies and bullets.

```rust [hl,8-9]
{{#include ../../my-game/examples/graphics-spaceship.rs:updatesprites}}
```

### Draw bullet animations

Now we can use the function `draw_texture_ex` to draw each frame of the
animation. Exchange the lines that draws a circle for each bullet to the code
below. First we call the method `frame()` on the `bullet_sprite` to get the
current animation frame and set it to the variable `bullet_frame`..

Inside the loop that draws all the bullets we'll call `draw_texture_ex` to
draw the bullet frame. It takes the `bullet_texture` as argument, and an `x`
and `y` position based on the size of the bullet. We also add the struct
`DrawTextureParams` with the fields `dest_size` and `source_rect`. The field
`dest_size` defines the size the texture will be drawn as, so we will use a
`Vec2` with the size of the bullet for both `x` and `y`. Finally we'll use
`bullet_frame.source_rect` which is a reference to where in the texture the
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
works the same as when drawing the bullets. First we'll retrieve the current
frame from the animation sprite, and then draw it using `draw_texture_ex`.

Because the spaceship animation isn't the same size in width and height we'll
use `ship_frame.dest_size` to get the size to draw. To make it a bit bigger
we'll double the size.

```rust
{{#include ../../my-game/examples/graphics-spaceship.rs:drawship}}
```

If everything works correctly there should be animated graphics for both the
spaceship and the bullets when running the game.

## Improve loading times

Adding the following snippet at the end of the `Cargo.toml` file will ensure
assets are loaded much faster.

```
[profile.dev.package.'*']
opt-level = 3
```

```admonish tip title="Challenge" class="challenge"
Try using the two extra spaceship animations to make the ship turn only
slightly just when it changes direction and then make it turn fully after a
short amount of time.
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

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/graphics-spaceship-and-bullets.toml}}

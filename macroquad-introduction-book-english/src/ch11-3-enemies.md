# Animated enemies

![Screenshot](images/graphics-enemies.gif#center)

The only thing left is to change the boring squares and replace them with some
more exciting graphics. This works the same as when animating the spaceship,
we load a texture, create an `AnimatedSprite`, and change how the enemies are
drawn to the screen.

## Implementation

### Load the texture

Load the texutre `enemy-small.png` and set the filter mode to
`FilterMode::Nearest`.

```rust [hl,1-4]
{{#include ../../mitt-spel/examples/graphics-enemies.rs:loadresources}}
```

### Create animation

![Enemy spritesheet](assets/enemy-small.png#pixelated)

Create an `AnimatedSprite` to describe the animations in the texture. It is
only one animation with two rames. The graphics for the small enemy is 16x16
pixels, but the texture has one pixel gutter between the frames to ensure that
they don't bleed into each other when we scale the texture.

```rust
{{#include ../../mitt-spel/examples/graphics-enemies.rs:sprite}}
```

### Update animation

The enemy sprites needs to be updated after the animations for the spaceship
and bullets.

```rust [hl,3]
{{#include ../../mitt-spel/examples/graphics-enemies.rs:updatesprites}}
```

### Draw enemy frames

We can now change the drawing of squares to drawing the texture from the
current frame of the animation. Vi retrieve the frame from
`enemy_small_sprite` and use the `source_rect` in `DrawTextureParams` in the
`draw_texture_ex()` call. Since the enemies have a randomized size, we'll use
the size of the enemy when setting the `dest_size` and `x` and `y`
coordinates.

```rust [hl,1,3-13]
{{#include ../../mitt-spel/examples/graphics-enemies.rs:drawenemy}}
```

We have now change to graphics for all the elements of the game and when you
run it now it should look like a real game.

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../mitt-spel/examples/graphics-enemies.rs:all}}
```
</details>
</div>


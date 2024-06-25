# Graphical explosions

![Screenshot](images/graphics-explosions.gif#center)

To make the explosions a bit more spectacular we will add graphical textures
to the particles.

## Implementation

### Import

To begin with, we need to update the import of `macroquad_particles`
and replace `ColorCurve` with `AtlasConfig`.

```rust
{{#include ../../my-game/examples/graphics-explosion.rs:import}}
```

### Update the particle configuration

We need to update the particle configuration for our `particle_explosion` so
that it will use `AtlasConfig` to make it use a texture to draw the particles
instead of using the `ColorCurve`. We also update the size and lifetime to
work better with the graphics.

The `AtlasConfig` describes the layout of the spritesheet when animating
particles with a texture. The arguments to `new()` are `n` for columns, `m`
for rows, and a `range` for start and end index of the animation. Our
spritesheet has five frames in a single row, and we want to use them all for
our animation, so we use the values `5`, `1`, and the range `0..`.

```rust [hl,10,12,14]
{{#include ../../my-game/examples/graphics-explosion.rs:emitterconfig}}
```

### Load textures

![Explosion spritesheet](assets/explosion.png#pixelated)

Before the line that builds the texture atlas we need to load the texture with
the animation for the particle explosion. The file is called `explosion.png`.
Don't forget to set the filter on the texture to `FilterMode::Nearest`.

```rust [hl,1-4]
{{#include ../../my-game/examples/graphics-explosion.rs:loadresources}}
```

### Add the texture

When we create the explosion, we need to add the texture to use. We'll also
update the number to get a few more particles. We need to use the method
`clone()` on the texture, which is efficient since it is only a pointer to
the texture.

```rust [hl,3-4]
{{#include ../../my-game/examples/graphics-explosion.rs:explosiontexture}}
```

When the game is run, the explosions will be animated with the explosion
image instead of colored squares.

```admonish tip title="Challenge" class="challenge"
Change the values of `EmitterConfig` fields based on the size of the enemy
that is hit.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/graphics-explosion.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/graphics-explosions.toml}}

</div>

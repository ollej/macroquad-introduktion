# Graphical explosions

![Screenshot](images/graphics-explosions.gif#center)

To make the explosions a bit more spectacular we will add graphical textures
to the particles.

## Implementation

### Import

To begin with we need to update the import of `macroquad_particles`
and replace `ColorCurve` with `AtlasConfig`.

```rust
{{#include ../../mitt-spel/examples/graphics-explosion.rs:import}}
```

### Update the particle configuration

We need to update the particle confguration for our `particle_explosion` so
that it will use `AtlasConfig` to make it use a texture to draw the particles
instead of using the `ColorCurve`. We also update the size and lifetime to
work better with the graphics.

```rust [hl,10,12,14]
{{#include ../../mitt-spel/examples/graphics-explosion.rs:emitterconfig}}
```

### Load textures

![Spritesheet för explosionen](assets/explosion.png#pixelated)

Before the line that builds the texture atlas we need to load the texture with
the animation for the particle explosion. The file is called `explosion.png`.
Don't forget to set the filter on the texture to `FilterMode::Nearest`.

```rust [hl,1-4]
{{#include ../../mitt-spel/examples/graphics-explosion.rs:loadresources}}
```

### Add the texture

When we create the explosion we need to add the texture to use. We'll also
update the amount to get a few more particles. We need to use the method
`clone()` on the texture, which is a efficient since it is only a pointer to
the texture.

```rust [hl,3-4]
{{#include ../../mitt-spel/examples/graphics-explosion.rs:explosiontexture}}
```

When the game is run the explosions should be animated with the explosion
image instead of colored squares.

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../mitt-spel/examples/graphics-explosion.rs:all}}
```
</details>
</div>

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/explosions.toml}}

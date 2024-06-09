# Particle explosions

![Screenshot](images/particle-explosions.gif#center)

We don't want the squares to just disappear when they are hit by a bullet. So
now we'll make use of the Macroquad particle system to generate explosions.
The particle system is used to easily create and draw many small particles on
the screen based on a base configuration. In our case the particles will start
from the center of the square and move outwards in all directions. In a later
chapter we will add a texture to the particles to make it look even more like
a real explosion.

## Implementation

### Add the particle crate

The code for Macroquads particle system is in a separate crate. Start by
adding it to the `Cargo.toml` file, either by changing the file by hand, or by
running the following command:

```sh
cargo add macroquad-particles
```

The following line will be added to the `Cargo.toml` file under the heading
`[dependencies]`.

```toml [hl,10]
[package]
name = "my-game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
macroquad = { version = "0.4" }
macroquad-particles = "0.2.1"
```

```admonish bug
Version 0.2.1 of macroquad-particles doesn't support the latest version of
Macroquad. If you get an error when compiling you can try to use both
macroquad and macroquad-particles 
[directly from git](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).
```

### Import crate

At the top of `main.rs` we need to import the things we use from the
`macroquad_particles` module.

```rust
{{#include ../../my-game/examples/particle-explosions.rs:import}}
```

### Particle configuration

We'll use the same configuration for all the explosions, and will only change
the size based on the sizes of the squares. Create a function that returns an
`EmitterConfig` that can be used to create an `Emitter`. The `Emitter` is a
point from where particles can be generated.

```rust
{{#include ../../my-game/examples/particle-explosions.rs:particleconfig}}
```

```admonish info
There are a lot of different things to configure in an `Emitter`. The fields of 
[`EmitterConfig`](https://docs.rs/macroquad-particles/latest/macroquad_particles/struct.EmitterConfig.html)
is described in the documentation of the module `macroquad-particles´.
```

### Vector of explosions

We need another vector to keep track of all the explosions. It includes a
tuple with an `Emitter` and the coordinate it should be drawn at.

```rust
{{#include ../../my-game/examples/particle-explosions.rs:explosions}}
```

When we start a new game we need to clear the vector of explosions.

```rust [hl,4]
{{#include ../../my-game/examples/particle-explosions.rs:clearexplosions}}
```

### Create an explosion

When a ghastly square is hit by a bullet we'll create a new `Emitter` based on
the configuration from `particle_explosion()` with the addition that the
number of particles is based on the size of the square. The coordinate that
the particles will be generated at should be the same as the coordinates of
the square.

```rust [hl,8-14]
{{#include ../../my-game/examples/particle-explosions.rs:addexplosion}}
```

### Removing explosions

When the emitter has finished drawing all the particles we need to remove them
from the `explosions` vector so that we'll stop trying to draw it. Add the
following code below the code that removes squares and bullets.

```rust
{{#include ../../my-game/examples/particle-explosions.rs:removeexplosions}}
```

### Drawing explosions

After drawing all the squares we can loop through the `explosions` vector and
draw them. We only need to send in the coordinate the particles will be
generated at, then the emitter will randomize and move all the particles by
itself.

```rust
{{#include ../../my-game/examples/particle-explosions.rs:drawexplosion}}
```

It's time to try the game to see if there are particle explosions when the
squares get hit by bullets.

```admonish tip title="Challenge" class="challenge"
Read the documentation for `EmitterConfig` and try what happens if you change
different values. Can you add a particle system that shoots particles out the
back of the circle so it looks like a rocket exhaust.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/particle-explosions.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/particle-explosions.toml}}

</div>

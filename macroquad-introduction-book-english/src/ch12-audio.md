# Music and sound effects

A game doesn't only need graphics to be good, it also needs to sound good.
Let's add some music and sound effects to the game.

## Implementation

### Activate the sound feature

To be able to use sound in Macroquad we need to activate the `audio` feature.
This is done by adding `audio` to the list of features in the macroquad
dependency in the `Cargo.toml` file.

```toml [hl,9]
[package]
name = "my-game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
macroquad = { version = "0.4", features = ["audio"] }
macroquad-particles = "0.2.2"
```

### Import

The sound module isn't included the Macroquad prelude, so we need to import
the `audio` module at the top of the `main.rs` file. The things we need to
import are `load_sound`, `play_sound`, `play_sound_once`, and
`PlaySoundParams`.

```rust
{{#include ../../my-game/examples/audio.rs:import}}
```

### Load resources

After all the textures have been loaded, we can load the music and sound
effects. There is a file with the music that is called `8bit-spaceshooter.ogg`
and two `wav` files with sound effects, `explosion.wav` and `laser.wav`. The
music is in the file format Ogg Vorbis which is supported by most, but not
all, web browsers.

```rust
{{#include ../../my-game/examples/audio.rs:loadresources}}
```

```admonish note title="Safari sounds"
In order for the music to work on the Safari web browser it has to be
converted to `WAV` format. This would make the file very large, so another
option is to use a version in `OGG` format and one in `MP3` and select which
one to use based on the web browser being used.
```

### Play music

Before the game loop begins we will start playing the music. This is done with
the function `play_sound()`, which takes a sound, and the struct
`PlaySoundParams` as arguments. In the parameters we set the sound to be
played in a loop and with full volume.

```rust
{{#include ../../my-game/examples/audio.rs:playmusic}}
```

```admonish info title="Stop the music"
To stop the music use the function `stop_sound()` which takes the sound as
argument.
```

### Play laser sound

When the player is shooting a bullet, we will play the sound effect of a laser
blast using the function `play_sound_once()`. This function takes the sound to
play as the argument. It is a shortcut instead of using `play_sound()` with a
non-looping parameter.

```rust [hl,8]
{{#include ../../my-game/examples/audio.rs:playlaser}}
```

```admonish info title="Turn up the volume"
It's also possible to set the sound volume per sound using the function
`set_sound_volume()` which takes a sound and a number between `0` and `1` as
argument.
```

### Play explosion sound

When a bullet hits an enemy, we will play the explosion sound, also using the
function `play_sound_once()`.

```rust [hl,14]
{{#include ../../my-game/examples/audio.rs:playexplosion}}
```

You can now start the game, and it should play music and sound effects.

```admonish tip title="Challenge" class="challenge"
It might be a bit intense to start the music at full volume. Try setting the
volume lower at the start and increase it once the game starts. Maybe also try
to stop the music when the player pauses the game.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/audio.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/audio.toml}}

</div>

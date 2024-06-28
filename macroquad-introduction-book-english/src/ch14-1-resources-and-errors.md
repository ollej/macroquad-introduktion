# Resources and errors

In this chapter we will refactor our code without adding any new functionality
to the game. We do this to build a foundation to be able to add a
loading screen during the loading of resources in the web version. We also
want to be able to refactor all the drawing to be done by the structs. Finally
we will be able to move code away from our `main` function which is starting
to get a bit hard to follow.

## Implementation

### Resources struct

We start by creating a new struct called `Resources` that will contain all the
files we load from the file system. Add it above the `main` function. The
struct will have a field for every asset loaded.

```rust
{{#include ../../my-game/examples/resources-and-errors.rs:struct}}
```

### Resources impl

Directly below the `Resources` struct we'll add an implementation block for it.
To begin with it will only contain a `new` method that loads all the files and
returns an instance of the struct if everything went as expected. We'll reuse
the code that used to be in the `main` function to load all the files.

We'll also store the UI `Skin` as a resource so we won't have to return the
font and all the images used for it.

The difference in the code is that we've replaced all the `unwrap()` and
`expect()` calls to use the `?` operator instead. Using this the error will be
returned instead of exiting the program. This means we will be able to handle
the error in a single place in our `main` function if we want to. The error
message is an enum of the type `macroquad::Error`.

```admonish info title="More information"
The errors available in Macroquad are documented in
[macroquad::Error](https://docs.rs/macroquad/latest/macroquad/enum.Error.html).
```

```rust
{{#include ../../my-game/examples/resources-and-errors.rs:impl}}
```

### Returning errors

To keep things as simple as possible we'll let our `main` function return a
result that may be an error. This means we can use the `?` operator in the
`main` function as well. If the `main` function returns an error, the game
will quit and the error message will be printed on the console.

The standard return value for the `main` function is `()`, which is the Rust
unit type that can be used if no value will be returned. Before when the
function didn't specify a return value, this was still returned implicitly.

If the last expression in a function ends with a semi colon (`;`) the return
value will be skipped and `()` is returned instead.

```rust [hl,2]
{{#include ../../my-game/examples/resources-and-errors.rs:main}}
```

```admonish info title="More information"
If you want to know how the Rust unit type works you can find more information
in the [Rust unit documentation](https://doc.rust-lang.org/std/primitive.unit.html).
```

### Remove `unwrap()`

When loading the material for the shader we used to use the method `unwrap()`
which we will now change to the `?` operator to return any error instead. This
change is in the last line of the code below.

```rust [hl,13]
{{#include ../../my-game/examples/resources-and-errors.rs:material}}
```

### Load resources

We've finally reached the most interesting part of this chapter. It's time to
change the code that loads file assets to instead instantiate our `Resources`
struct. We add the result to the `resources` variable that we can use later
when we need to use a resource.

Note that we use `await` after the `new()` method as it is async. We also use
the `?` operator to bubble up any errors.

```rust [hl,2]
{{#include ../../my-game/examples/resources-and-errors.rs:loadresources}}
```

### Update resource usages

Now that we have loaded all the assets with the `Resources` struct we need to
update all the places that uses a resource so that they retrieve the asset
from it instead. We basically just add `resources.` in front of every resource
name.

#### Game music

```rust [hl,2]
{{#include ../../my-game/examples/resources-and-errors.rs:theme}}
```

#### User interface

Now that we've saved the UI `Skin` in our `Resources` struct we only need to
activate it using `root_ui().push_skin()`. We can replace all the lines
that builds the UI with a single line.

```rust [hl,1]
{{#include ../../my-game/examples/resources-and-errors.rs:ui}}
```

#### Laser sound

The laser sound needs to use the `resources` variable.

```rust [hl,9]
{{#include ../../my-game/examples/resources-and-errors.rs:sound_laser}}
```

#### Explosions

We need to update both the texture and the sound for the explosions.

```rust [hl,4,9]
{{#include ../../my-game/examples/resources-and-errors.rs:sound_explosion}}
```

#### Bullets

Update the call to drawing bullets to use the texture from `resources`.

```rust [hl,3]
{{#include ../../my-game/examples/resources-and-errors.rs:bullet_texture}}
```

#### Spaceship

The spaceship also needs to use the texture from `resources`.

```rust [hl,3]
{{#include ../../my-game/examples/resources-and-errors.rs:ship_texture}}
```

#### Enemies

When the enemies are drawn, we need to add `resources` as well.

```rust [hl,3]
{{#include ../../my-game/examples/resources-and-errors.rs:enemy_small_texture}}
```

That's everything that needs to be changed this time. In this chapter we've
created a struct that contains all the loaded assets that we use when drawing
textures and playing sounds.

```admonish tip title="Challenge" class="challenge"
Instead of just exiting the game when encountering an error you could try to
display the error message on the screen using the `draw_text()` function of
Macroquad. Remember that the program will then need to keep on running and do
nothing but displaying the text.
```

## Try the game

The game should work exactly like before.

```admonish info title="More information"
Sometimes the cargo dependencies can become out of sync. Some users have
experienced this in this chapter. The symptoms are that the buttons in the
main menu starts to "glitch" and it requires multiple clicks to press the
buttons. A workaround for this issue is to rebuild all the dependencies using
`cargo clean`.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/resources-and-errors.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/resources-and-errors.toml}}

</div>

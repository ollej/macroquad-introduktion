# Starfield shader

<div class="noprint">

![Screenshot](images/screenshots-web/starfield-shader.gif#center)

</div>
<div class="onlyprint">

![Screenshot](images/screenshots-print/starfield-shader.png#center)

</div>

The purple background on the screen is starting to feel a bit boring. Instead
we'll add something more interesting. We'll use a pixel shader to display a
moving starfield in the background. How to implement a shader is outside the
scope of this guide, so we'll use one that has already been prepared for us.

In short, a shader is a small program that runs on the GPU of the computer.
They are written in a C-like programming language called GLSL. The shader is
made up of two parts, a vertex shader and a fragment shader. The vertex shader
converts from coordinates in a 3D environment to the 2D coordinates of the
screen. Whereas the fragment shader is run for every pixel on the screen to
set the variable `gl_FragColor` to define the color that pixel should have.
Since our game is entirely in 2D, the vertex shader won't do anything other
than setting the position.

## Implementation

### Shaders

At the top of the `main.rs` file we'll add a vertex shader, the fragment
shader will be loaded from a file that we will add later. We'll use the Rust
macro `include_str!()` to read the file as a `&str` at compile time. The
vertex shader is so short that it can be added directly in the Rust source
code.

The most important line in the vertex shader is the line that sets
`gl_Position`. For simplicity's sake we'll also set the `iTime` variable that
is used by the fragment shader from `_Time.x`. It would also be possible to
use `_Time` directly in the fragment shader, but it would mean we have
to change it slightly.

```rust
{{#include ../../my-game/examples/starfield-shader.rs:shaders}}
```

### Initialize the shader

In the `main()` function, above the loop, we need to setup a few variables to
be able to use the shader. We start by adding the variable
`direction_modifier` that will be used to change the direction of the stars
horizontally, depending on whether the circle is moved left or right. After
that we create a `render_target` to which the shader will be rendered.

Now we can create a `Material` with the vertex shader and the fragment shader
using the enum `ShaderSource::Glsl`.

In the parameters we'll also setup two uniforms for the shader that are global
variables that we can set for every frame. The uniform `iResolution` will
contain the size of the window and `direction_modifier` is used to control the
direction of the stars.

```rust
{{#include ../../my-game/examples/starfield-shader.rs:setupshader}}
```

```admonish info title="Available uniforms"
Macroquad will automatically add some uniforms to all shaders. The available
uniforms are `_Time`, `Model`, `Projection`, `Texture`, and `_ScreenTexture`.
```

### Draw the shader

It's now time to change the purple background to our new starfield. Change the
line `clear_background(DARKPURPLE);` to the code below.

The first thing we need to do is to set the window resolution to the material
uniform `iResolution`. We'll also set the `direction_modifier` uniform to the
same value as the corresponding variable.

After this we'll use the function `gl_use_material()` to use the material.
Finally we can use the function `draw_texture_ex()` to draw the texture from
our `render_target` on the background of the screen. Before we continue we'll
restore the shader with the function `gl_use_default_material()` so that it
won't be used when drawing the rest of the game.

```rust
{{#include ../../my-game/examples/starfield-shader.rs:drawshader}}
```

### Controlling the stars

When the player holds down the left or right arrow key we'll add or subtract a
value from the variable `direction_modifier` so that the shader can control
the movement of the stars. Remember to multiply the value with `delta_time` so
that the change is relative to framerate, just like when doing the movement.

 ```rust [hl,3,7]
{{#include ../../my-game/examples/starfield-shader.rs:shaderdir}}
```

### Create the shader file

Now create a file with the name `starfield-shader.glsl` in the `src` directory
to contain the fragment shader and add the following code:

```glsl
{{#include ../../my-game/examples/starfield-shader.glsl}}
```

```admonish info title="Shader Coding tutorial video"
If you want to know how the shader works you can watch the video
[Shader Coding: Making a starfield](https://youtu.be/rvDo9LvfoVE) by The Art of Code.
```

Our starfield is now done and the game is starting to look like it takes place
in outer space.

```admonish tip title="Challenge: Star colors" class="challenge"
Look at the video Shader Coding: Making a starﬁeld and see if you can change
the color and size of the stars.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/starfield-shader.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/starfield-shader.toml}}

</div>

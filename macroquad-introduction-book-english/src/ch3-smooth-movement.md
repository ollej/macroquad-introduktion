# Smooth movement

<div class="noprint">

![Screenshot](images/screenshots-web/smooth-movement.gif#center)

</div>
<div class="onlyprint">

![Screenshot](images/screenshots-print/smooth-movement.png#center)

</div>

Since Macroquad will draw frames as quickly as possible, we need to check how
much time has passed between each update to determine how far the circle
should move. Otherwise, our game will run at different speeds on different
computers, depending on how quickly they can run the application. The specific
framerate will depend on your computer; if Vsync is enabled it may be locked
to 30 or 60 frames per second.

## Implementation

We will expand the application and add a constant that determines how quickly
the circle should move. We call the constant `MOVEMENT_SPEED` and assign the
value `200.0`. If the circle moves too fast or too slow, we can decrease or
increase this value.

```rust
{{#include ../../my-game/examples/smooth-movement.rs:constant}}
```

### Time between frames

Now we will use the function `get_frame_time()` to get the time in seconds
that has passed since the last frame. We assign this value to a variable
called `delta_time` that we will use later.

```rust
{{#include ../../my-game/examples/smooth-movement.rs:deltatime}}
```

### Update movement

When the variables `x` and `y` are updated, we will multiply the values of 
the constant `MOVEMENT_SPEED` by the variable `delta_time` to get how far the
circle should move during this frame.

```rust [hl,2,5,8,11]
{{#include ../../my-game/examples/smooth-movement.rs:movement}}
```

### Limit movement

Finally, we will prevent the circle from moving outside of the window. 
We use the Macroquad function `clamp()` to make sure `x` and `y` are never
below `0` or above the width of the window.

```rust
{{#include ../../my-game/examples/smooth-movement.rs:clamp}}
```

```admonish info
The `clamp()` function is used to clamp a value between a minimum and maximum
value. It is part of the Macroquad
[Math API](https://docs.rs/macroquad/latest/macroquad/math/index.html).
```

```admonish tip title="Challenge" class="challenge"
Change the constant `MOVEMENT_SPEED` if the circle is moving too slow or too
fast.

What do you need to change to ensure that the entire circle stays within the
window when the position is clamped?
```

<div class="no-page-break">

## Source

The code should now look like this:

```rust
{{#include ../../my-game/examples/smooth-movement.rs:all}}
```
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/smooth-movement.toml}}

</div>

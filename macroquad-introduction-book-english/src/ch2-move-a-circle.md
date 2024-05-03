# Fly away

![Screenshot](images/move-a-circle.gif#center)

A game is not much fun without something happeing on the screen. To begin
with, we will show a circle that we can steer with the arrow keys on the
keyboard.

## Implementation

The first two lines of the `main` function uses the functions `screen_width()`
and `screen_height()` to get the width and height of the application window.
These values are divided by `2` to get the coordinates of the center of the
window, and stored in the variables `x` and `y`. These variables will be used
to decide where to draw the circle on the screen.

```rust
{{#include ../../mitt-spel/examples/move-a-circle.rs:coordinates}}
```

### Handle keyboard input

Inside the main loop we will still clear the background as it should be done
on each frame. After that there are four `if` statements that check if any of
the arrow keys on the keyboard has been pressed. The variables `x` and `y` are
changed to move the circle in the corresponding direction.

The function `is_key_down()` returns true if the given key is being
pressed during the current frame. The argument is of the enum `KeyCode` that
contains all keys available on the keyboad.

```rust
{{#include ../../mitt-spel/examples/move-a-circle.rs:movement}}
```

```admonish info
You can find which other keys are available in the 
[documentation of KeyCode](https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html).
```

### Draw a circle

Finally we will draw a circle on the screen at the coordinates in `x` and `y`.
The circle has a radius of 16 and will be drawn with a yellow color.

```rust
{{#include ../../mitt-spel/examples/move-a-circle.rs:draw}}
```

```admonish info
Macroquad has several constants for common
[colors](https://docs.rs/macroquad/latest/macroquad/color/colors/index.html),
and you can also use the macro 
[`color_u8`](https://docs.rs/macroquad/latest/macroquad/macro.color_u8.html)
to create a color with specific values for red, green, blue and transparency.

The other shapes that can be drawn with Macroquad is described in the
documentation of Macroquad's 
[Shape API](https://docs.rs/macroquad/latest/macroquad/shapes/index.html).
```

```admonish tip title="Challenge" class="challenge"
Change the value added to `x` and `y` to increase or decrease the speed the
circle will move.
```

<div class="no-page-break">

## Source

The source of `main.rs` should look like this:

```rust
{{#include ../../mitt-spel/examples/move-a-circle.rs:all}}
```

When you run the game there will be a yellow circle drawn in the middle of the
window. Try using the arrow keys to move around the circle.
</div>

## Quiz

Try your newfound knowledge by answering this quiz before continuing.

{{#quiz ../quizzes/move-a-circle.toml}}

# Collisions

<div class="noprint">

![Screenshot](images/screenshots-web/collision.gif#center)

</div>
<div class="onlyprint">

![Screenshot](images/screenshots-print/collision.png#center)

</div>

To make the game more exciting, let's add some conflict. If our hero, the
brave yellow circle, collides with a square, the game will be over and has to
be restarted.

After we have drawn the circle and all squares, we'll add a check to see if
any square touches the circle. If it does, we'll display the text "`GAME
OVER!`" in capital letters and wait for the player to press the space key.
When the player presses space, we'll reset the vector with squares and move
the circle back to the center of the screen.

## Implementation

### Collision function

We expand the `Shape` struct with an implementation that contains the method
`collides_with()` to check if it collides with another `Shape`. This method uses
the `overlaps()` helper method from Macroquad's
[`Rect`](https://docs.rs/macroquad/latest/macroquad/math/struct.Rect.html)
struct. We also create a helper method called `rect()` that creates a `Rect`
from our Shape.

```admonish info title="The Macroquad Rect struct"
There are many methods on `Rect` to do calculations on rectangles, such as
`contains()`, `intersect()`, `scale()`, `combine_with()` and `move_to()`.
```

```rust
{{#include ../../my-game/examples/collision.rs:implshape}}
```

```admonish note title="Rect origin"
The origin of Macroquad's `Rect` is also from the top left corner, so we must
subtract half its size from both `X` and `Y`.
```

### Is it game over?

Let's add a boolean variable called `gameover` to the start of the main loop
to keep track of whether the player has died.

```rust
{{#include ../../my-game/examples/collision.rs:variable}}
```

Since we don't want the circle and squares to move while it's game over, the
movement code is wrapped in an `if` statement that checks if the `gameover`
variable is `false`.

```rust
        if !gameover {
            ...
        }
```

### Collision

After the movement code, we add a check if any square collides with the
circle. We use the method `any()` on the iterator for the vector `squares` and
check if any square collides with our hero circle. If a collision occurs, we
set the variable `gameover` to true.

```rust
{{#include ../../my-game/examples/collision.rs:collision}}
```

```admonish tip title="Challenge" class="challenge"
The collision code assumes that the circle is a square. Try writing code that
takes into account that the circle does not entirely fill the square.
```

### Reset the game

If the `gameover` variable is `true` and the player has just pressed the space
key, we clear the vector `squares` using the `clear()` method and reset the
`x` and `y` coordinates of `circle` to the center of the screen. Then, we set the
variable `gameover` to `false` so that the game can start over.

```rust
{{#include ../../my-game/examples/collision.rs:gameover}}
```

```admonish info title="Key press difference"
The difference between the functions `is_key_down()` and `is_key_pressed()` is
that the latter only checks if the key was pressed during the current frame,
while the former returns true for all frames from when the button was pressed
and then held down. An experiment you can do is to use `is_key_pressed()` to
control the circle.

There's also a function called `is_key_released()` which checks if the key was
released during the current frame.
```

### Display GAME OVER

Finally, we draw the text "GAME OVER!" in the middle of the screen after the
circle and squares have been drawn, but only if the variable `gameover` is `true`.
Macroquad does not have any feature to decide which things will be drawn on
top of other things. Each thing drawn will be drawn on top of all other
things drawn earlier during the the same frame.

```admonish info title="Extended text parameters"
It's also possible to use the function
[`draw_text_ex()`](https://docs.rs/macroquad/latest/macroquad/text/fn.draw_text_ex.html)
which takes a
[`DrawTextParams` struct](https://docs.rs/macroquad/latest/macroquad/text/struct.TextParams.html)
instead of `font_size` and `color`. Using that struct it's possible to set
more parameters such as `font`, `font_scale`, `font_scale_aspect` and `rotation`.
```

```rust
{{#include ../../my-game/examples/collision.rs:drawgameover}}
```

```admonish tip title="Challenge" class="challenge"
Since `draw_text()` is based on the text's baseline, the text won't appear
exactly in the center of the screen. Try using the `offset_y` and `height` fields
from `text_dimensions` to calculate the text's midpoint. Macroquad's example
[text measures](https://github.com/not-fl3/macroquad/blob/master/examples/text_measures.rs)
can provide tips on how it works.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/collision.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/collision.toml}}

</div>

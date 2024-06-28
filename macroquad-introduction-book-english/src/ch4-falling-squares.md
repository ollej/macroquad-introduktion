# Falling squares

![Screenshot](images/falling-squares.gif#center)

To make sure there is something happening in our game, it's time to create
some action. Since the hero in our game is a brave circle, our opponents will
be squares falling down from the top of the window.

## Implementation

### Struct for shapes

To keep track of our circle and all the squares, we'll create a struct that we
can name `Shape`, which will contain the size and speed, as well as `x` and `y`
coordinates.

```rust
{{#include ../../my-game/examples/falling-squares.rs:shape}}
```

### Initialize random number generator

We'll use a random number generator to determine when new squares should
appear on the screen, how big they should be and how fast they will move.
Therefore, we need to seed the random generator so that it doesn't produce the
same random numbers every time. This is done at the beginning of the `main`
function using the `rand::srand()` method, to which we pass the current time
as the seed.

```rust
{{#include ../../my-game/examples/falling-squares.rs:srand}}
```

```admonish note title="Miniquad in play!"
We are using the function `miniquad::date::now()` from the
[graphics library Miniquad](https://docs.rs/miniquad/latest/miniquad/index.html)
to get the current time.
```

### Vector of squares

At the beginning of the `main` function we create a vector called `squares`
that will contain all the squares to be displayed on the screen. The new
variable `circle` will represent our hero, the amazing circle. The speed
uses the constant `MOVEMENT_SPEED`, and the `x` and `y` fields are set to the
center of the screen.

```rust
{{#include ../../my-game/examples/falling-squares.rs:variables}}
```

Start by modifying the program so that `circle` is used instead of the
variables `x` and `y` and confirm that everything works as it did before
adding the enemy squares.

```admonish note title="Getting a “type annotation needed” warning?"
The Rust compiler might warn about "type annotations needed" on the Vector.
Once we add an enemy square in the next section that warning should disappear.
```

### Add enemy squares

It's time to start the invasion of evil squares. Here, just like before, we
split updating the movement and drawing the squares. This way, the movement
does not depend on the screen's refresh rate, ensuring that all changes are
done before we start drawing anything to the screen.

First, we use the function `rand::gen_range()` to determine whether to add a
new square. It takes two arguments, a minimum value and a maximum value, and
returns a random number between those two values. We generate a random number
between 0 and 99, and if the value is 95 or higher, a new `Shape` is created
and added to the `squares` vector. To add some variation, we also use
`rand::gen_range()` to get different size, speed, and starting position of
every square.

```rust
{{#include ../../my-game/examples/falling-squares.rs:generatesquare}}
```

```admonish note title="The coordinate system starts top left"
Rectangles are drawn starting from their upper left corner. Therefore, we
subtract half of the square's size when calculating the `x` position. The
`y` position starts at a negative value of the square's size, so it starts
completely outside the screen.
```

### Update square positions

Now we can iterate through the vector using a for loop and update the
Y position using the square's speed and the variable `delta_time`. This will
make the squares move downwards across the screen.

```rust
{{#include ../../my-game/examples/falling-squares.rs:movesquares}}
```

### Remove invisible squares

Next, we need to clean up all the squares that have moved off the bottom of
the screen since it's unnecessary to draw things that are not visible. We'll
use the `retain()` method on the vector, which takes a function that determines
whether elements should be kept. We'll check if the square's `y` value is still
less than the height of the window plus the size of the square.

```rust
{{#include ../../my-game/examples/falling-squares.rs:removesquares}}
```

### Draw the squares

Finally, we add a `for` loop that iterates over the `squares` vector and uses
the function `draw_rectangle()` to draw a rectangle at the updated position
and with the correct size. Since rectangles are drawn with x and y from the
top-left corner and our coordinates are based on the center of the square, we
use some mathematics to calculate where they should be placed. The size is
used twice, once for the width of the square and once for the height. We set
the color to `GREEN` so that all squares will have a green color.

```rust
{{#include ../../my-game/examples/falling-squares.rs:drawsquares}}
```

```admonish note title="Fancier rectangles available"
It's also possible to use the function
[`draw_rectangle_ex()`](https://docs.rs/macroquad/latest/macroquad/shapes/fn.draw_rectangle_ex.html)
that uses the struct
[`DrawTextureParams`](https://docs.rs/macroquad/latest/macroquad/shapes/struct.DrawRectangleParams.html)
instead of a color. In addition to setting color, it can be used to set
`rotation` and `offset` of the rectangle.
```


```admonish tip title="Challenge" class="challenge"
Try setting a different color for each square by using the method `choose()`
on vectors from Macroquad's
[ChooseRandom trait](https://docs.rs/macroquad/latest/macroquad/rand/trait.ChooseRandom.html),
which returns a random element from the vector.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/falling-squares.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/falling-squares.toml}}

</div>

# Points

![Screenshot](images/points.gif#center)

What is a game without points and a high score? Now that the circle can shoot
down the squares it is time to add some points. Every square that is shot down
will add to the score, where bigger squares will be worth more points. The
current score will be shown on the screen, as well as the highest score
achieved.

```admonish info
Bigger squares could be worth more because they contain more resources. Later
on they could be made harder to destroy by being able to take more bullets
hits.
```

If the current score is the highest score when the game is over, it will be
written to a file on disk so that it can be read each time the game is
started. This will only work if the game is played on desktop as the
WebAssembly version doesn't have access to the file system. It would be
possible to store the high score in the browser storage, but that won't be
covered here to keep the implementation simple.

## Implementation

### Import module

To be able to read and write files we need to import [std::fs
modul](https://doc.rust-lang.org/std/fs/index.html) from the Rust standard
library. Add this line directly below the line to import Macroquad at the top
of the file.

```rust
{{#include ../../my-game/examples/points.rs:import}}
```

### New variables

We will need two new variables, `score` and `high_score`, to keep track of the
player's points as well as the highest score ever achieved. We'll use the
function `fs::read_to_string()` to read the file `highscore.dat` from disk.
The points stored in the file need to be converted to `u32` with
`i.parse::<u32>()`. If anything goes wrong, if the file doesn't exist or it
contains something other than a number, the number `0` will be returned
instead.

```rust
{{#include ../../my-game/examples/points.rs:variables}}
```

```admonish note
We're writing the points directly to the computers hard drive, which will not
work if the game has been compiled to WebAssembly and is run on a web page.
This will be treated as if the file doesn't exist.

It could be possible to use the browser's storage, or sending the score to a web
server, but that is not covered by this guide.
```

### Updating the high score

If the circle collides with a square we'll check if the current score is
higher than the high score. If it is higher, we'll update the high score and
store the new high score to the file `highscore.dat`.

```rust [hl,2-4]
{{#include ../../my-game/examples/points.rs:savepoints}}
```

```admonish note
Macroquad supports reading files when the game is run on a web page. We could
use the function
[`load_string()`](https://docs.rs/macroquad/latest/macroquad/file/fn.load_string.html)
to load the high score instead. But since it isn't possible to save the file,
this isn't particularly useful in this case.
```

### Increasing the score

When a bullet hits a square, we'll increase the current score based on the
size of the square. After that we'll update the `high_score` if the current
`score` is higher.

```rust [hl,4-5]
{{#include ../../my-game/examples/points.rs:points}}
```

### Resetting the score

When a new game is started, we need to set the `score` variable to `0`.

```rust [hl,6]
{{#include ../../my-game/examples/points.rs:clearpoints}}
```

### Displaying scores

Finally, we'll display the `score` and `high_score` on the screen. We'll
display the `score` in the top left corner of the screen. To be able to
display the high score in the top right corner we'll use the function
[`measure_text()`](https://docs.rs/macroquad/latest/macroquad/text/fn.measure_text.html)
to calculate how far from the right edge of the screen the text should be
displayed.

To ensure that the dimensions are correct we must use the same arguments for
both `measure_text()` and `draw_text()`. The arguments for these functions are
`text`, `font`, `font_size` and `font_scale`. Since we aren't setting any
specific font or scaling the size of the text, we'll use `None` as the value
for `font`, and `1.0` as `font_scale`. The `font_size` can be set to `25.0`.

```rust
{{#include ../../my-game/examples/points.rs:drawpoints}}
```

```admonish info
The function `measure_text()` returns the struct
[`TextDimensions`](https://docs.rs/macroquad/latest/macroquad/text/struct.TextDimensions.html)
which contains the fields `width`, `height`, and `offset_y`.
```

Run the game and try to get a high score!

```admonish tip title="Challenge" class="challenge"
Try writing a congratulations message below the "GAME OVER" text if the player
reached a high score.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/points.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/points-system.toml}}

</div>

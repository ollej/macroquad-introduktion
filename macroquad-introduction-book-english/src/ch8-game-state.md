# Game state

![Screenshot](images/game-state.gif#center)

Before we add any more functionality to our game it's time for some
refactoring. To make it easier to keep track of the game state we'll add an
enum called `GameState` with variants to differentiate between a game that is
played or if it is game over. Because of this we'll be able to remove the
`gameover` variable, and we can also add states for showing a start menu and
pausing the game.

## Implementation

### Game state enum

Begin by adding an enum called `GameState` below the `Shape` implementation.
It should contain all four possible game states: `MainMenu`, `Playing`,
`Paused`, and `GameOver`.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:stateenum}}
```

### Game state variable

Replace the line that declares the `gameover` variable with a line that
instantiates a `game_state` variable set to `GameState::MainMenu`. 

```rust
{{#include ../../mitt-spel/examples/game-state.rs:statevariable}}
```

### Match on GameState

We'll replace the old code in the game loop with code that matches on the
`game_state` variable. It has to match on all four states in the enum. Later
on we'll add back code from the earlier chapter within the matching arms. Keep
the call to clearing the screen at the start of the loop, and the call to
`next_frame().await` at the end.

```rust [hl,3-16]
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                ...
            }
            GameState::Playing => {
                ...
            }
            GameState::Paused => {
                ...
            }
            GameState::GameOver => {
                ...
            }
        }

        next_frame().await
```

### Main menu

Now it's time to add back code into the match arms to handle each game state.
When the game is started the state will be `GameState::MainMenu`. We'll start
by quitting the gmae if the `Escape` key is pressed. If the player presses the
space key we'll set the `game_state` to the new state `GameState::Playing`.
We'll also reset all the game variables. Finally we'll draw the text "Press
space" in the middle of the screen.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:mainmenu}}
```

### Playing the game

Let's add back the code for playing the game to the matching arm for the state
`GameState::Playing`. It's the same code as most of the game loop from the
last chapter. However, don't add back the code that handles game over as it
will be added in the matching arm for the `GameState::GameOver`.

We'll also add a code that checks if the player presses the `Escape` key and
change the state to `GameState::Paused`.

```rust [hl,1,24-26,108]
{{#include ../../mitt-spel/examples/game-state.rs:playing}}
```

### Pause the game

Many games have the option to pause the action, so we'll add support for that
in our game too. When the game is paused we'll check if the player presses
the `Escape` key and change the game state to `GameState::Playing` so that the
game can continue. We'll also draw a text on the screen showing that the game
is paused.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:paused}}
```

### Game Over

Finally we will handle what happens when the game is oer. If the player
presses the spacebar we'll change the state to `GameState::MainMenu` to allow
the player to start a new game or quit the game. We'll also draw the game over
text to the screen like we did in the last chapter.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:gameover}}
```

```admonish note
Since the states for `GameState::Playing` and `GameState::GameOver` are
separated nothing will be shown when the game is paused.
```

```admonish tip title="Challenge" class="challenge"
Now that we have main menu you could come up with a name for your game and
display it with a large font at the top of the screen in the state
`GameState::MainMenu`.

For extra credit you could also try to draw all the circles and squares even
when the game is paused.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../mitt-spel/examples/game-state.rs:all}}
```
</details>
</div>

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/game-state.toml}}
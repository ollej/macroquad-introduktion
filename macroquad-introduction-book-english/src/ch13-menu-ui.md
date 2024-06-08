# Graphical menu

![Screenshot](images/menu-ui.png#center)

Macroquad has a builtin system to display a graphical user interface where the
look can easily be changed using PNG images. We will use this to create a
graphical main menu for our game. There will be quite a lot of code to define
the look of the UI. However once that is done, it is very easy to use it.

The menu will have a window centered on the screen with the text "Main menu" in
the title bar. Inside the window there will be two buttons, one for "Play" and
one for "Quit". The UI will be built using different kinds of widgets such as
`label`, `button`, `editbox`, and `combobox`.

## Implementation 

To begin with we need to import what we need from the `ui` module.

```rust
{{#include ../../my-game/examples/menu-ui.rs:import}}
```

### Load resources

After loading the sounds we'll load the font and images used for the UI. 
There is an image to create the window, `window_background.png`, one image for
the buttons, `button_background.png`, and finally an image for when the button
is pressed, `button_clicked_background.png`. The images are loaded with the
function `load_image()` and binary files with the function `load_file()`. Both
images and files are loaded asynchronously and may return errors. This means
we will have to call `await` and `unwrap()` to get the files. If we can't load
the files needed to display the main menu we can just exit the program
immediately.

```rust
{{#include ../../my-game/examples/menu-ui.rs:loadresources}}
```

### Create a skin

Before the game loop we need to define how our UI should look. We will build
`Style` structs for the window, buttons and texts. After that we will use the
styles to create a `Skin`.

We use the function `root_ui()` that will draw widgets last in every frame
using a "default" camera and the coordinate system 
`(0..screen_width(), 0..screen_height())`.

#### Window look

To build a style we use a `StyleBuilder` that has helper methods to define all
the parts of the style. Vi get access to it by using the method
`style_builder()` on `root_ui()`. The values that aren't set will use the same
values as the default look.

We will use the method `background()` to set the image used to draw the
window. After that we can use `background_margin()` to define which parts of
the image that shouldn't be "stretched" out when the window changes size. This
is used to ensure that the edges of the window will look good.

The method `margin()` is used to set margins for the content. These values can
be negative to draw content on the borders of the window.

```rust
{{#include ../../my-game/examples/menu-ui.rs:windowstyle}}
```

```admonish info
There are a lot more methods to define styles, these are described in the
documentation for [Macroquad's
`StyleBuilder`](https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.StyleBuilder.html)
```

#### Button look

In the definition for buttons we'll use two images. Using `background()` we
set the default image for the button, and `background_clicked()` is used to
set the image to display while the button is clicked on.

We need to set both `background_margin()` and `margin()` to be able to stretch
the image to cover the text inside the button. The look of the text is defined
using the methods `font()`, `text_color()`, and `font_size()`.

```rust
{{#include ../../my-game/examples/menu-ui.rs:buttonstyle}}
```

#### Text look

Normal text displayed in the interface uses `label_style`. We will use the
same font as for the buttons, but with a slightly smaller font size.

```rust
{{#include ../../my-game/examples/menu-ui.rs:labelstyle}}
```

#### Define a Skin

We can now create a `Skin` using `window_style`, `button_style`, and
`label_style`. We won't define any other styles for the skin as we won't be
using them at the moment.

We set the current skin to use using `push_skin()`. We will only use one skin,
but to change between different looks between windows, it's possible to use
`push_skin()` and `pop_skin()` to change between them.

We will also set the variable `window_size` to define with size of the window.

```rust
{{#include ../../my-game/examples/menu-ui.rs:uiskin}}
```

```admonish info
It's possible to change the look of more parts of the UI, such as text boxes,
drop boxes etc. More information on how to do this can be found in the 
[documentation of the struct
Skin](https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.Skin.html).
```

### Build the menu

We can now build a menu by drawing a window with two buttons and a heading.
The content of the `GameState::MainMenu` matching arm can be replaced with the
code below.

Start vy creating a window using `root_ui().window()`. The function takes an
argument that is generated with the macro `hash!`, a position that we'll
calculate based on the window size and the screen dimensions, and finally a
`Vec2` for the size of the window. Finally it takes a function that is used to
draw the content of the window.

#### Window title

In the window function we start by setting a title of the window with the
widget `Label` that we can create using `ui.label()`. The method takes two
arguments, a `Vec2` for the position of the label and a string with the text
to display. It's possible to set `None` as position, in which case the
placement will be relative to the previous widget. We will use a negative `y`
position to place the text within the title bar of the window.

```admonish info
It's also possible to create widgets by instantiating a struct and using
builder methods.

`widgets::Button::new("Play").position(vec2(45.0, 25.0)).ui(ui);`
```

#### Buttons

After the label we'll add a button to begin playing the game. The method
`ui.button()` returns `true` when the button is clicked. We will use this to
set the `GameState::Playing` to start a new game.

Then we can create a button with the text "Quit" to exit the game.

```rust [hl,2-11,19-20,22-24]
{{#include ../../my-game/examples/menu-ui.rs:menu}}
```

```admonish info
There are many different widgets that can be used to create interfaces.
The list of available widgets can be found in the [documentation of the
struct `Ui`](https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.Ui.html).
```

## Try the game

When starting the game a graphical menu will be shown where the player can
choose to start a game or quit the program.

<div class="noprint no-page-break">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/menu-ui.rs:all}}
```
</details>
</div>

{{#quiz ../quizzes/menu-ui.toml}}

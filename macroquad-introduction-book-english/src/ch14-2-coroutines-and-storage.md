# Coroutines and Storage

When there are a lot of assets to load, it might take a while to load
everything. This is especially true for the WebAssembly version that loads
files via HTTP in the browser on a slow internet connection. In these cases we
want to display a loading message on the screen instead of just having a
completely black screen.

To solve this we will use something called `coroutines`, which will emulate
multitasking using the event loop in the browser. For the desktop these will
execute immediately instead. This can be used to handle state machines and
things that need to be evaluated over time. Using a coroutine we can load all
the resources in the background while also drawing to the screen.

Finally we will place the resources in the Macroquad `storage` that is a
global persistent storage. It can be used to save game configuration that
needs to be available anywhere in the game code without having to send the
data around.

```admonish info
Both `coroutines` and `storage` are experimental features of Macroquad and the
usage might change in future versions.
```

## Implementation 

### Importing

Let's start by importing `coroutines::start_coroutine` and
`collections::storage` from Macroquad's experimental namespace.

```rust
{{#include ../../my-game/examples/coroutines-and-storage.rs:import}}
```

### Create a new load method

Now we can create a `load()` method in the implementation block for the
`Resources` struct. In this method we'll add the code that loads the assets
using a coroutine and display a text message on the screen showing that
resources are being loaded.

The function `start_coroutine` takes an `async` block and returns a
`Coroutine`. Inside the async block we will instantiate the `Resources` struct
that loads all the assets. After that we use the `storage::store()` to save
the resources in the Macroquad storage. This will ensure that we can access
the resources anywhere in the code.

Using the method `is_done()` on `Coroutine` we can check if the couroutune has
finished running or not. We add a loop that runs until `is_done()` returns
`true`. While the coroutine is running we use `draw_text()` to display a
message on the screen.  We also add 1 to 3 periods after the text using the
code `".".repeat(((get_time() * 2.) as usize) % 4)`. We also need to use
`clear_background()` and `next_frame.await` inside the loop for everything to
work properly.

```rust
{{#include ../../my-game/examples/coroutines-and-storage.rs:load}}
```

```admonish info
More information about the Macroquad
[coroutines](https://docs.rs/macroquad/latest/macroquad/experimental/coroutines/index.html)
and
[storage](https://docs.rs/macroquad/latest/macroquad/experimental/collections/storage/index.html)
can be found in the Macroquad documentation.
```

### Loading assets

The call to loading resources needs to be updated to use the new `load()`
method instead of using `new()` directly. Since `load()` stores the resources
in the Macroquad storage we will use `storage::get::<Resources>()` to retrieve
the resources.

```rust [hl,2-3]
{{#include ../../my-game/examples/coroutines-and-storage.rs:loadresources}}
```

## Try the game

While the game is loading in a browser, the message "Loading resources..." will
be shown on the screen.

```admonish tip title="Challenge" class="challenge"
Make a loading spinner by including an image as bytes and draw it using the
`rotation` field in `DrawTextureParams` in the `load()` function instead of
displaying text.
```

<div class="noprint">

## Full source code

<details>
  <summary>Click to show the the full source code</summary>

```rust
{{#include ../../my-game/examples/coroutines-and-storage.rs:all}}
```
</details>
</div>

<div class="noprint">

## Quiz

Try your knowledge by answering the following quiz before you move on to the
next chapter.

{{#quiz ../quizzes/coroutines-and-storage.toml}}

</div>

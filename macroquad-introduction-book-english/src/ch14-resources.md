# Resources

![Black image with the text "Loading resources" followed by three blinking dots](images/coroutines-and-storage.gif#center)

We're starting to get quite a lot of code in our `main` function so it's time
to refactor again to get a little better code structure.

We'll start by moving all the loading of file assets to a struct. At the same
time we will change all the `unwrap()` and `expect()` calls to using the `?`
operator to handle error messages.

After that we will make use of a coroutine to load the resources in the
background while also displaying a message about loading resources on the
screen.

Finally we will use a `Storage` struct to make the resources available in the
code without having to send them around to every function where they are
needed.

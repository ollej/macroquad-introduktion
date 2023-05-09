use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}

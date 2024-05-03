use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}

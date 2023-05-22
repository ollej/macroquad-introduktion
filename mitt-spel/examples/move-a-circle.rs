// ANCHOR: all
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    // ANCHOR: coordinates
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    // ANCHOR_END: coordinates

    loop {
        clear_background(DARKPURPLE);

        // ANCHOR: movement
        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }
        // ANCHOR_END: movement

        // ANCHOR: draw
        draw_circle(x, y, 16.0, YELLOW);
        // ANCHOR_END: draw

        next_frame().await
    }
}
// ANCHOR_END: all

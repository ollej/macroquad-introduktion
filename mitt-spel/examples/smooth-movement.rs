// ANCHOR: all
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    // ANCHOR: constant
    const MOVEMENT_SPEED: f32 = 200.0;
    // ANCHOR_END: constant

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        // ANCHOR: deltatime
        let delta_time = get_frame_time();
        // ANCHOR_END: deltatime
        // ANCHOR: movement
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        // ANCHOR_END: movement

        // ANCHOR: clamp
        x = x.min(screen_width()).max(0.0);
        y = y.min(screen_height()).max(0.0);
        // ANCHOR_END: clamp

        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
}
// ANCHOR_END: all

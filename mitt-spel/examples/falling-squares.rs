// ANCHOR: all
use macroquad::prelude::*;

// ANCHOR: shape
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
// ANCHOR_END: shape

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 100.0;

    // ANCHOR: srand
    rand::srand(miniquad::date::now() as u64);
    // ANCHOR_END: srand
    // ANCHOR: variables
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    // ANCHOR_END: variables

    loop {
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }

        // Clamp X and Y to be within the screen
        circle.x = circle.x.min(screen_width()).max(0.0);
        circle.y = circle.y.min(screen_height()).max(0.0);

        // Generate a new square
        // ANCHOR: generatesquare
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }
        // ANCHOR_END: generatesquare

        // Move squares
        // ANCHOR: movesquares
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }
        // ANCHOR_END: movesquares

        // Remove squares below bottom of screen
        // ANCHOR: removesquares
        squares.retain(|square| square.y < screen_width() + square.size);
        // ANCHOR_END: removesquares

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        // ANCHOR: drawsquares
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        // ANCHOR_END: drawsquares

        next_frame().await
    }
}
// ANCHOR_END: all

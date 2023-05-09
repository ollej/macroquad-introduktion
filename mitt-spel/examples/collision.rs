use macroquad::{prelude::*, rand::*};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 100.0;

    srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut circle = Shape {
        size: 30.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        clear_background(DARKPURPLE);

        // Check for collissions
        let mut gameover = squares.iter().any(|square| circle.collides_with(square));
        if gameover && is_key_down(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }
        if !gameover {
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
            if gen_range(0, 99) >= 95 {
                let size = gen_range::<f32>(15.0, 40.0);
                let square = Shape {
                    size,
                    speed: gen_range::<f32>(50.0, 150.0),
                    x: gen_range::<f32>(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                };
                squares.push(square);
            }

            // Move squares
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Remove squares below bottom of screen
            squares.retain(|square| square.y < screen_width() + square.size);
        }

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        if gameover {
            let text = "Game Over!";
            let text_dimensions = measure_text(text, None, 60, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}

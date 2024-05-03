use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

#[macroquad::main("Flicker test")]
async fn main() {
    set_pc_assets_folder("assets");

    let ship_texture: Texture2D = load_texture("ship.png").await.expect("Couldn't load file");
    ship_texture.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    let mut ship_sprite = AnimatedSprite::new(
        16,
        24,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "left".to_string(),
                row: 2,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "right".to_string(),
                row: 4,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );
    ship_sprite.set_animation(0);

    loop {
        clear_background(BLACK);

        ship_sprite.update();
        let ship_frame = ship_sprite.frame();
        draw_texture_ex(
            &ship_texture,
            screen_width() / 2.0 - ship_frame.dest_size.x,
            screen_height() / 2.0 - ship_frame.dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(ship_frame.dest_size * 2.0),
                source: Some(ship_frame.source_rect),
                ..Default::default()
            },
        );

        next_frame().await
    }
}

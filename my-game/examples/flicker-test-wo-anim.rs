use macroquad::prelude::*;

#[macroquad::main("Flicker test")]
async fn main() {
    let texture: Texture2D = load_texture("assets/ship-one-frame.png").await.unwrap();
    //build_textures_atlas();

    loop {
        clear_background(BLACK);

        draw_texture(
            &texture,
            screen_width() / 2.0 - texture.width(),
            screen_height() / 2.0 - texture.height(),
            WHITE,
        );

        next_frame().await
    }
}

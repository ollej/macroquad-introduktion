use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

#[macroquad::main("Mitt spel")]
async fn main() {
    set_pc_assets_folder("assets");
    let window_background = macroquad::texture::load_image("window_background.png")
        .await
        .unwrap();
    let window_style = root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();
    let button_background = macroquad::texture::load_image("button_background.png")
        .await
        .unwrap();
    let button_clicked_background = macroquad::texture::load_image("button_clicked_background.png")
        .await
        .unwrap();
    let font = load_file("atari_games.ttf").await.unwrap();
    let button_style = root_ui()
        .style_builder()
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(64)
        .build();
    let label_style = root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(32)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    loop {
        let window_size = vec2(280.0, 320.0);
        root_ui().window(
            hash!(),
            vec2(
                screen_width() / 2.0 - window_size.x / 2.0,
                screen_height() / 2.0 - window_size.y / 2.0,
            ),
            window_size,
            |ui| {
                widgets::Button::new("Play")
                    .position(vec2(20.0, 25.0))
                    .ui(ui);
                widgets::Button::new("Quit")
                    .position(vec2(20.0, 125.0))
                    .ui(ui);
            },
        );
        next_frame().await
    }
}

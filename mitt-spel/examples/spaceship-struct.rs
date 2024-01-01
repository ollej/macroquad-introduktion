// ANCHOR: all
use macroquad::prelude::*;
use macroquad_particles::{self as particles, ColorCurve, Emitter, EmitterConfig};

use std::fs;

const FRAGMENT_SHADER: &'static str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

trait Collidable {
    fn collides_with(&self, other: &dyn Collidable) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect;
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Collidable for Shape {
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

// ANCHOR: shipstruct
struct Ship {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
    direction_modifier: f32,
    texture: Texture2D,
}

impl Ship {
    fn new(texture: Texture2D) -> Self {
        Ship {
            size: 32.0,
            speed: 150.0,
            x: 0.0,
            y: 0.0,
            collided: false,
            direction_modifier: 0.0,
            texture,
        }
    }

    fn reset(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.collided = false;
    }

    fn move_right(&mut self, delta_time: f32) {
        self.x += self.speed * delta_time;
        self.direction_modifier += 0.05 * delta_time;
        self.clamp();
    }
    fn move_left(&mut self, delta_time: f32) {
        self.x -= self.speed * delta_time;
        self.direction_modifier -= 0.05 * delta_time;
        self.clamp();
    }
    fn move_down(&mut self, delta_time: f32) {
        self.y += self.speed * delta_time;
        self.clamp();
    }
    fn move_up(&mut self, delta_time: f32) {
        self.y -= self.speed * delta_time;
        self.clamp();
    }

    fn clamp(&mut self) {
        // Clamp X and Y to be within the screen
        self.x = self.x.min(screen_width()).max(0.0);
        self.y = self.y.min(screen_height()).max(0.0);
    }

    fn bullet(&self) -> Shape {
        Shape {
            x: self.x,
            y: self.y,
            speed: self.speed * 2.0,
            size: 5.0,
            collided: false,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.x - self.size / 2.0,
            self.y - self.size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 48.0)),
                source: Some(Rect::new(0.0, 0.0, 16.0, 24.0)),
                ..Default::default()
            },
        );
    }
}

impl Collidable for Ship {
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

// ANCHOR_END: shipstruct

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.25,
        lifetime_randomness: 0.7,
        explosiveness: 0.95,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: RED,
            mid: ORANGE,
            end: RED,
        },
        ..Default::default()
    }
}

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    // ANCHOR: newship
    let ship_texture: Texture2D = load_texture("assets/ship.png")
        .await
        .expect("Couldn't load file");
    let mut ship = Ship::new(ship_texture);
    // ANCHOR_END: newship
    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);
    let mut game_state = GameState::MainMenu;

    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_owned(), UniformType::Float2),
                ("direction_modifier".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    loop {
        clear_background(BLACK);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", ship.direction_modifier);
        gl_use_material(material);
        draw_texture_ex(
            render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    // ANCHOR: resetship
                    ship.reset();
                    // ANCHOR_END: resetship
                    score = 0;
                    game_state = GameState::Playing;
                }
                let text = "Tryck på mellanslag";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::Playing => {
                let delta_time = get_frame_time();
                // ANCHOR: moveship
                if is_key_down(KeyCode::Right) {
                    ship.move_right(delta_time);
                }
                if is_key_down(KeyCode::Left) {
                    ship.move_left(delta_time);
                }
                if is_key_down(KeyCode::Down) {
                    ship.move_down(delta_time);
                }
                if is_key_down(KeyCode::Up) {
                    ship.move_up(delta_time);
                }
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(ship.bullet());
                }
                // ANCHOR_END: moveship
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Generate a new square
                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(15.0, 40.0);
                    squares.push(Shape {
                        size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                        y: -size,
                        collided: false,
                    });
                }

                // Movement
                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                // Remove shapes outside of screen
                squares.retain(|square| square.y < screen_height() + square.size);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

                // Remove collided shapes
                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);

                // Remove old explosions
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // Check for collisions
                if squares.iter().any(|square| ship.collides_with(square)) {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.size.round() as u32 * 2,
                                    ..particle_explosion()
                                }),
                                vec2(square.x, square.y),
                            ));
                        }
                    }
                }

                // Draw everything
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
                }
                // ANCHOR: drawship
                ship.draw();
                // ANCHOR_END: drawship
                for square in &squares {
                    draw_rectangle(
                        square.x - square.size / 2.0,
                        square.y - square.size / 2.0,
                        square.size,
                        square.size,
                        GREEN,
                    );
                }
                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }
                draw_text(
                    format!("Poäng: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High score: {}", high_score);
                let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimensions.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Playing;
                }
                let text = "Pausad";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "Game Over!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
            }
        }

        next_frame().await
    }
}
// ANCHOR_END: all

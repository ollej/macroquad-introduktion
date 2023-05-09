# Skjuta

Det känns lite orättvist att vår stackars cirkel inte kan försvara sig mot de
läskiga fyrkanterna. Därför är det dags att implementera skott som cirkeln kan
skjuta ner fyrkanterna med.

För att hålla reda på vilka fyrkanter som har blivit träffade av kulor så
lägger vi till ett nytt fält `collided` av typen `bool` i `Shape`.

```rust
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}
```

Vi måste ha en ny vektor som håller reda på alla kulor som har skjutits. Vi
kallar den `bullets` och skapar den efter vektorn med `squares`. Här anger vi
vilken typ vektorn ska innehålla eftersom Rust-kompilatorn måste veta vilken
typ det är innan vi har tilldelat den något värde. Vi använder structen
`Shape` även för kulorna för enkelhetens skull.

```rust
    let mut bullets: Vec<Shape> = vec![];
```

Efter vi har kollat om cirkeln har kolliderat med en fyrkant lägger vi till en
kontroll om någon fyrkant blir träffad av en kula. Både kulan och fyrkanten
uppdatera och collided sätts till `true` så att vi kan ta bort dom längre ned
i koden.

```rust
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }
```

Efter cirkeln har förflyttats så lägger vi till en kontroll om spelaren har
tryck på mellanslag, och lägger till en kula i vektorn med kulor. Kulans x-
och y-koordinater sätts till samma som cirkeln, och hastigheten till dubbla
cirkelns hastighet.

```rust
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: MOVEMENT_SPEED * 2.0,
                    size: 5.0,
                    collided: false,
                });
            }
```

Även kulorna behöver tas bort om dom hamnar utanför skärmen.

```rust
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);
```

Nu är det dags att ta bort alla fyrkanter och kulor som har kolliderat med
något. Det gör vi enkelt med `retain`-metoden och behåller alla som inte har
`collided` satt till `true`. Vi gör detta på båda vektorerna för `squares` och
`bullets`.

```rust
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);
```

Innan vi ritar ut cirkeln så ritar vi ut alla kulor, så att dom hamnar under
övriga former.

```rust
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }
```

Det var allt för att kunna skjuta sönder fyrkanter.

## Kompletta källkoden

Källkoden för vårt spel ska nu se ut så här:

```rust
use macroquad::{prelude::*, rand::*};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
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
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 30.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };

    loop {
        clear_background(DARKPURPLE);

        // Check for collissions
        let mut gameover = squares.iter().any(|square| circle.collides_with(square));
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }

        if gameover && is_key_down(KeyCode::Space) {
            squares.clear();
            bullets.clear();
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
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: MOVEMENT_SPEED * 2.0,
                    size: 5.0,
                    collided: false,
                });
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
                    collided: false,
                };
                squares.push(square);
            }

            // Movement
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // Remove shapes outside of screen
            squares.retain(|square| square.y < screen_width() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            // Remove collided shapes
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);
        }

        // Draw everything
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }
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
```

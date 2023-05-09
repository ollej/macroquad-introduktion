## Mjukare rörelser

Eftersom Macroquad kommer rita bildrutor så snabbt som den kan så måste vi
kolla hur lång tid som har gått mellan varje uppdatering för att avgöra hur
långt cirkeln ska förflyttas. Annars kommer vårt spel gå olika fort på olika
datorer, beroende på hur snabbt dom kan köra programmet.

Vi ska därför utöka programmet och lägga till en konstant variabel som avgör
hur snabbt cirkeln ska röra sig. Vi kallar den `MOVEMENT_SPEED` och börjar med
att tilldela den värden `50.0`. Går det för fort eller för sakta kan vi sänka
eller öka detta värde.

```rust
    const MOVEMENT_SPEED: f32 = 100.0;
```

Därefter använder vi funktionen `get_frame_time()` som ger oss hur lång tid
det har gått sedan föregående bildruta ritades på skärmen och tilldelar den
till variabeln `delta_time`.

```rust
        let delta_time = get_frame_time();
```

Förändringen av variablerna `x` och `y` kan sedan bytas ut till en
multiplikation av värdena för `MOVEMENT_SPEED` och `delta_time` för att få hur
långt cirkeln ska förflyttas under denna bildruta.

```rust
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
```

Slutligen vill vi också att cirkeln aldrig ska hamna utanför fönstret, därför
begränsar vi variablerna `x` och `y`.

```rust
        x = x.min(screen_width()).max(0.0);
        y = y.min(screen_height()).max(0.0);
```

### Källkod

Nu ser vårt program ut så här:

```rust
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 100.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();
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

        x = x.min(screen_width()).max(0.0);
        y = y.min(screen_height()).max(0.0);

        draw_circle(x, y, 15.0, YELLOW);
        next_frame().await
    }
}
```



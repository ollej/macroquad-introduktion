## Far å flyg

Ett spel är inte så roligt utan att det händer något på skärmen. Till att
börja med visar vi en boll som vi kan styra med knapptryckningar.

De första två raderna i main-funktionen använder funktionerna `screen_width()`
och `screen_height()` för att få bredden och höjden på fönstret. Dessa värden
delas med 2 för att få koordinaterna till mitten av skärmen, och tilldelas
till variablerna `x` och `y`.

```rust
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
```

Inne i loopen rensar vi fortfarande skärmen, vilket måste göras vid varje
bildruta. Därefter kommer fyra if-satser som kollar om piltangerna är
nedtryckta och ändrar på variablerna `x` eller `y` som avgör var cirkeln ska
visas. Funktionen `is_key_down()` returnerar sant om den angivna tangenten är
nedtryckt. Dess argument är enumen `KeyCode` som innehåller alla tangenter som
finns på ett tangentbord. Slutligen ritas cirkeln ut på de angivna
koordinaterna med en radie på 15 och med gul färg.

```rust
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
```

### Källkod

Byt ut innehållet i `main.rs` till följande kod:

```rust
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

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

        draw_circle(x, y, 15.0, YELLOW);
        next_frame().await
    }
}
```

När du kör programmet så kommer det visas en gul cirkel i mitten av skärmen.
Prova att använda piltangenterna för att flytta omkring bollen.



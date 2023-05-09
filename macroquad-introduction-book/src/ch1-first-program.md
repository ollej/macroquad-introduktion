## Ditt första Macroquad-program

Nu är det dags att programmera ditt första program med Macroquad. Börja med
att installera programmeringsspråket Rust om du inte redan har gjort det.
Skapa därefter ett nytt Rust-projekt med Cargo och lägg till `macroquad` som
beroende.

```sh
cargo new --bin mitt-spel
cd mitt-spel/
cargo add macroquad
```

Öppna sedan filen `src/main.rs` i din favorit-editor och ändra innehållet till
följande kod:

```rust
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
```

Kör programmet med `cargo run` så ska ett nytt fönster med mörklila bakgrund
öppnas efter att kompileringen är klar.

### Beskrivning av programmet

Första raden används för att importera allt som behövs från Macroquad, vilket
enklast görs med `use macroquad::prelude::*`, men det går också att importera
alla funktioner manuellt.

Attributet `#[macroquad::main("Mitt spel")]` används för att berätta för
Macroquad vilken funktion som ska köras. Macroquad kommer skapa ett fönster
med titeln som anges som argument, och exekvera main-funktionen asynkront.

Inne i main-funktionen körs en evig loop som aldrig avslutas. Inne i loopen
ligger all spellogik som ska köras varje bildruta. I vårt fall rensar vi
bakgrunden till mörklila med funktionen `clear_background(DARKPURPLE)`. I
slutet av loopen används funktionen `next_frame().await` som kommer blocka
exekveringen tills nästa bildruta.


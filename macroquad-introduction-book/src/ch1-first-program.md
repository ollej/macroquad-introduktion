# Ditt första Macroquad-program

Nu är det dags att programmera ditt första program med Macroquad. Börja med
att installera programmeringsspråket Rust om du inte redan har gjort det.

![Screenshot](images/first-program.png#center)

## Implementering

Skapa ett nytt Rust-projekt med Cargo och lägg till `macroquad` med version
0.4 som beroende.

```sh
cargo new --bin mitt-spel
cd mitt-spel/
cargo add macroquad@0.4
```

Din `Cargo.toml` fil kommer nu se ut såhär:

```toml
[package]
name = "mitt-spel"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
macroquad = "0.4"
```

Öppna filen `src/main.rs` i din favorit-editor och ändra innehållet till
följande kod:

```rust
{{#include ../../mitt-spel/examples/first-program.rs}}
```

Kör programmet med `cargo run` så ska ett nytt fönster med mörklila bakgrund
öppnas efter att kompileringen är klar.

## Beskrivning av programmet

Första raden används för att importera allt som behövs från Macroquad, vilket
enklast görs med `use macroquad::prelude::*`, men det går också att importera
alla funktioner manuellt.

Attributet `#[macroquad::main("Mitt spel")]` används för att berätta för
Macroquad vilken funktion som ska köras. Macroquad kommer skapa ett fönster
med titeln som anges som argument, och exekvera main-funktionen asynkront.

```admonish info
För att ändra andra inställningar för fönstret, som storlek eller om det ska
visas i fullskärm, går det att använda structen
[Conf](https://docs.rs/macroquad/latest/macroquad/window/struct.Conf.html).
```

Inne i main-funktionen körs en evig loop som aldrig avslutas. Inne i loopen
ligger all spellogik som ska köras varje bildruta. I vårt fall rensar vi
bakgrunden till mörklila med funktionen `clear_background(DARKPURPLE)`. I
slutet av loopen används funktionen `next_frame().await` som kommer blocka
exekveringen tills nästa bildruta.

```admonish note title="Notera" 
Även om `clear_background()` inte används explicit så kommer Macroquad att rensa
skärmen i början av varje bildruta.
```

```admonish tip title="Utmaning" class="challenge"
Prova att ändra vilken bakgrundsfärg fönstret ska ha till din favoritfärg.
```

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/first-program.toml}}

# Poängsystem

![Screenshot](images/points.gif#center)

Vad vore ett spel utan poäng och high score? Det är nu dags att implementera
ett poängsystem för vårt spel. Poäng kommer ges för varje fyrkant som skjuts
ner, baserat på storleken. Poängen kommer visas på skärmen, såväl som den
högsta poäng som har uppnåtts. Om poängen är en high score så kommer poängen
skrivas ner till en fil på disk så att det kan läsas in igen nästa gång spelet
startas.

## Implementering

### Importera modul

För att kunna läsa och skriva filer behöver vi importera [Rusts std::fs
module](https://doc.rust-lang.org/std/fs/index.html) som innehåller
funktionalitet för att läsa och skriva till datorns lokala filsystem. Denna
rad kan läggas in under raden som importerar Macroquad längst upp i filen.

```rust
{{#include ../../my-game/examples/points.rs:import}}
```

### Nya variabler

Vi behöver två nya variabler, `score` och `high_score` för att hålla reda på
spelarens poäng och den högsta poängen som har uppnåtts. Vi använder oss av
funktionen `fs::read_to_string()` för att läsa in filen `highscore.dat`.
Poängen i filen måste konverteras till en `u32` vilket görs med
`i.parse::<u32>()`. Om något går fel, som att filen inte finns eller
innehåller något som inte är en siffra, så kommer siffran `0` att returneras.

```rust
{{#include ../../my-game/examples/points.rs:variables}}
```

```admonish note title="Notera"
Här skriver vi direkt till datorns hårddisk, vilket inte fungerar om spelet
har kompilerats till WASM och körs på en webbsida.
```

### Uppdatera high score

Om cirkeln krockar med en fyrkant så lägger vi till en kontroll om spelarens
poäng är en high score. Är den det så skriver vi ner high scoren till filen
`highscore.dat`.

```rust [hl,2-4]
{{#include ../../my-game/examples/points.rs:savepoints}}
```

```admonish note title="Notera"
Macroquad har stöd för att läsa filer som fungerar även när spelet körs på en
webbsida. Här skulle vi kunna använda funktionen
[`load_string()`](https://docs.rs/macroquad/latest/macroquad/file/fn.load_string.html)
istället, men eftersom vi inte kan skriva filen är det inte så meningsfullt.
```

### Öka poäng

Om en kula träffar en fyrkant så ökar vi spelarens poäng baserat på storleken
på fyrkanten. Sen uppdaterar vi värdet i variabeln `high_score` om poängen är
högre än det gamla värdet.

```rust [hl,4-5]
{{#include ../../my-game/examples/points.rs:points}}
```

### Nollställ poäng

När vi startar en ny spelomgång måste vi nollställa variabeln `score`.

```rust [hl,6]
{{#include ../../my-game/examples/points.rs:clearpoints}}
```

### Skriv ut poäng och high score

Till sist ritar vi ut poängen och high score på skärmen. Poängen skriver vi
alltid ut i övre vänstra hörnet. För att kunna skriva ut high scoren i högra
hörnet behöver vi använda oss av funktionen
[`measure_text()`](https://docs.rs/macroquad/latest/macroquad/text/fn.measure_text.html)
för att räkna ut hur långt från skärmens högra sida texten ska placeras.

För att dimensionerna ska stämma måste samma värden användas som argument till
`measure_text()` som till `draw_text()`. Argumenten är `text`, `font`,
`font_size` och `font_scale`. Eftersom vi inte sätter någon speciell font
eller skalar om texten så skickar vi in `None` som `font`, och `1.0` som
`font_scale`. Däremot måste `font_size` vara samma som i anropet av
`draw_text()` vilket i vårt fall är `25.0`.

```rust
{{#include ../../my-game/examples/points.rs:drawpoints}}
```

```admonish info
Funktionen `measure_text()` returnerar structen
[`TextDimensions`](https://docs.rs/macroquad/latest/macroquad/text/struct.TextDimensions.html)
som innehåller fälten `width`, `height` och `offset_y`.
```

Kör igång spelet och försök få så hög poäng som möjligt!

```admonish tip title="Utmaning" class="challenge"
Testa att skriva ut en gratulationstext på skärmen vid Game Over om spelaren
uppnådde en high score.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../my-game/examples/points.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/points-system.toml}}

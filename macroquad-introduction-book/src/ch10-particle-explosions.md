# Partikelexplosioner

![Screenshot](images/particle-explosions.gif#center)

Vi vill inte att fyrkanterna bara ska försvinna i tomma intet när dom träffas
av en kula. Därför ska vi nu använda oss av Macroquads partikelsystem för att
generera explosioner. Partikelsystemet kan effektivt skapa och rita många små
partiklar på skärmen baserat på en grundkonfiguration. I vårt fall kommer
partiklarna att åka ut från fyrkantens mittpunkt i alla riktningar. Vi kan
senare lägga på en textur för att göra det ännu mer explosionsliknande.

## Implementering

### Lägg till crate

Koden för Macroquads partikelsystem ligger i en egen crate, därför behöver vi
lägga till den i vår `Cargo.toml` fil. Det kan göras antingen genom att ändra
i filen eller att köra följande kommando.

```sh
cargo add macroquad-particles
```

Följande rad kommer att läggas till i filen `Cargo.toml` under rubriken
`[dependencies]`.

```toml [hl,10]
[package]
name = "mitt-spel"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
macroquad = { version = "0.4", features = ["audio"] }
macroquad-particles = "0.2.0"
```

```admonish warning
Version 0.2.0 av macroquad-particles stödjer inte senaste versionen av
Macroquad. Om du får felet `"error[E0574]: expected struct, variant or union
type, found enum 'ShaderSource'"` måste du använda macroquad och
macroquad-particles [direkt från
git](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories)
tills detta är åtgärdat.
```

### Importera crate

Överst i `main.rs` måste vi importera det vi använder från paketet
`macroquad_particles`.

```rust
{{#include ../../mitt-spel/examples/particle-explosions.rs:import}}
```

### Partikelkonfiguration

Vi kommer använda samma konfiguration för alla explosioner, och kommer bara
ändra dess storlek baserat på fyrkantens storlek. Därför skapar vi en funktion
som returnerar en `EmitterConfig` som kan användas för att skapa en `Emitter`.
En `Emitter` är en punkt utifrån partiklar kan genereras.

```rust
{{#include ../../mitt-spel/examples/particle-explosions.rs:particleconfig}}
```

```admonish info
Det finns en mängd sätt att konfigurera en `Emitter`. Fälten för
[`EmitterConfig`](https://docs.rs/macroquad-particles/latest/macroquad_particles/struct.EmitterConfig.html)
finns beskrivna i dokumentationen för modulen macroquad-particles.
```

### Vektor med explosioner

Vi behöver en vektor för att hålla reda på alla explosioner som inträffar. Den
innehåller en tuple med en Emitter och koordinaten som den ska ritas ut på.

```rust
{{#include ../../mitt-spel/examples/particle-explosions.rs:explosions}}
```

När vi startar ett nytt spel behöver vi rensa vektorn med explosioner.

```rust [hl,4]
{{#include ../../mitt-spel/examples/particle-explosions.rs:clearexplosions}}
```

### Skapa en explosion

När en fyrkant träffas av en kula så skapar vi en ny `Emitter` baserat på
konfigurationen från `particle_explosion()` med tillägget att antalet
partiklar som ska genereras baseras på fyrkantens storlek. Koordinaten som
partiklarna ska genereras ifrån sätts till samma som fyrkantens koordinater.

```rust [hl,8-14]
{{#include ../../mitt-spel/examples/particle-explosions.rs:addexplosion}}
```

### Ta bort explosioner

När emittern har ritat färdigt alla partiklar så måste vi ta bort den ur
vektorn `explosions` då vi inte ska rita ut den längre. Lägg till denna kod
efter fyrkanterna och kulorna har tagits bort.

```rust
{{#include ../../mitt-spel/examples/particle-explosions.rs:removeexplosions}}
```

### Rita ut explosioner

Efter att fyrkanterna har ritats ut kan vi gå igenom vektorn med explosioner
och rita ut dem. Vi behöver bara skicka in vilken koordinat partiklarna ska
genereras på, sedan hanterar emittern själv att slumpa fram alla partiklarna
och flytta på dem.

```rust
{{#include ../../mitt-spel/examples/particle-explosions.rs:drawexplosion}}
```

Prova spelet och se om det blir explosioner när fyrkanterna beskjuts.

```admonish tip title="Utmaning" class="challenge"
Läs dokumentationen för `EmitterConfig` och prova vad som händer om du ändrar
olika värden. Kan du lägga till ett partikelsystem som skjuter ut rakt bakåt
från cirkeln så att det ser ut som en raketflamma.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/particle-explosions.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/particle-explosions.toml}}

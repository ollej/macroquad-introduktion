# Musik och ljudeffekter

Ett spel behöver inte bara grafik för att det ska bli bra. Det behövs även
musik och ljudeffekter. 

## Implementering

### Aktivera feature för ljud

För att kunna använda ljud i Macroquad måste en feature aktiveras för dess
crate i `Cargo.toml` filen för ditt spel. Uppdatera raden för `macroquad`
under rubriken `[dependencies]` till att inkludera featuren `audio`.

```toml [hl,9]
[package]
name = "my-game"
version = "0.1.0"
edition = "2024"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
macroquad = { version = "0.4", features = ["audio"] }
macroquad-particles = "0.2.0"
```

### Importera

Ljudmodulen är inte inkluderad i Macroquads `prelude`, därför behöver vi
importera det vi använder i modulen `audio` längst upp i källkoden.

```rust
{{#include ../../my-game/examples/audio.rs:import}}
```

### Ladda in resurser

Efter att alla texturer är inladdade så kan vi ladda in musiken och
ljudeffekter. Vi har en mp3-fil med musiken som heter `8bit-spaceshooter.ogg`
och två `wav`-filer med ljudeffekter, `explosion.wav` och `laser.wav`. Musiken
använder filformatet Ogg Vorbis som stöds av det mesta, dock inte av vissa
webbläsare.

```rust
{{#include ../../my-game/examples/audio.rs:loadresources}}
```

### Spela upp musik

Innan loopen börjar vi spela upp musiken. Det görs med `play_sound`,
som tar ett ljud, och structen `PlaySoundParams`. Vi sätter att ljudet ska
spelas loopande, och med full volym.

```rust
{{#include ../../my-game/examples/audio.rs:playmusic}}
```

```admonish info
För att stoppa musiken kan man använda funktionen `stop_sound()` som tar ett
ljud som argument.
```

### Spela laserljud

När spelaren skjuter en ny kula så spelar vi upp ett laserljud med hjälp av
funktionen `play_sound_once()` som tar det ljud som ska spelas upp som
argument. Det är en genväg för att slippa använda `PlaySoundParams` för att
spela upp ett ljud som inte loopar.

```rust [hl,8]
{{#include ../../my-game/examples/audio.rs:playlaser}}
```

```admonish info
Det går även att sätta ljudvolym per ljud med hjälp av funktionen
`set_sound_volume()` som tar ett ljud och ett tal mellan 0 och 1.
```

### Spela explosionsljud

När en kula träffar en fiende spelar vi upp explosionsljudet, även detta med
`play_sound_once`.

```rust [hl,14]
{{#include ../../my-game/examples/audio.rs:playexplosion}}
```

När du startar spelet bör det nu spela upp musik och ljudeffekter.

```admonish tip title="Utmaning" class="challenge"
Det kanske är lite intensivt att musiken börjar på full volym direkt, prova
att sänka volymen i början och höj den när spelet börjar. Prova även att
stoppa musiken när spelaren pausar spelet.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../my-game/examples/audio.rs:all}}
```
</details>
</div>


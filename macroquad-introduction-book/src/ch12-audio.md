# Musik och ljudeffekter

Ett spel behöver inte bara grafik för att det ska bli bra. Det behövs även
musik och ljudeffekter. 

Ljudmodulen är inte inkluderad i Macroquads `prelude`, därför behöver vi
importera det vi använder i modulen `audio` längst upp i källkoden.

```rust
{{#include ../../mitt-spel/examples/audio.rs:import}}
```

Efter att alla texturer är inladdade så kan vi ladda in musiken och
ljudeffekter. Vi har en mp3-fil med musiken som heter `8bit-spaceshooter.ogg`
och två `wav`-filer med ljudeffekter, `explosion.wav` och `laser.wav`. Musiken
använder filformatet Ogg Vorbis som stöds av det mesta, dock inte av vissa
webbläsare.

```rust
{{#include ../../mitt-spel/examples/audio.rs:loadresources}}
```

Innan loopen börjar vi spela upp musiken. Det görs med `play_sound`,
som tar ett ljud, och structen `PlaySoundParams`. Vi sätter att ljudet ska
spelas loopande, och med full volym.

```rust
{{#include ../../mitt-spel/examples/audio.rs:playmusic}}
```

```admonish info
För att stoppa musiken kan man använda funktionen `stop_sound()` som tar ett
ljud som argument.
```

När spelaren skjuter en ny kula så spelar vi upp ett laserljud med hjälp av
funktionen `play_sound_once()` som tar det ljud som ska spelas upp som
argument. Det är en genväg för att slippa använda `PlaySoundParams` för att
spela upp ett ljud som inte loopar.

```rust
{{#include ../../mitt-spel/examples/audio.rs:playlaser}}
```

```admonish info
Det går även att sätta ljudvolym per ljud med hjälp av funktionen
`set_sound_volume()` som tar ett ljud och ett tal mellan 0 och 1.
```

När en kula träffar en fiende spelar vi upp explosionsljudet, även detta med
`play_sound_once`.

```rust [hl,14]
{{#include ../../mitt-spel/examples/audio.rs:playexplosion}}
```

När du startar spelet bör det nu spela upp musik och ljudeffekter.

```admonish tip
Det kanske är lite intensivt att musiken börjar på full volym direkt, prova
att sänka volymen i början och höj den när spelet börjar. Prova även att
stoppa musiken när spelaren pausar spelet.
```

## Kompletta källkoden

Källkoden för vårt spel ska nu se ut så här:

```rust
{{#include ../../mitt-spel/examples/audio.rs:all}}
```


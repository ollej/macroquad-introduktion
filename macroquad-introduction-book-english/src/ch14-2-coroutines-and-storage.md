# Coroutines och Storage

När spel har mycket resurser att ladda in så kan det ta en stund innan det har
laddat klart. Speciellt om spelet kör i en browser och användaren sitter på en
dålig internetuppkoppling. Då vill vi inte att det bara ska visas en svart
skärm, utan istället ska ett meddelande visas för användaren.

För att lösa det ska vi använda oss av något som heter `coroutines` som
emulerar multitasking. Det kan användas för att hantera tillståndsmaskiner och
saker som behöver evalueras över tid. Med hjälp av en coroutine kan vi ladda
alla resurser i bakgrunden och samtidigt uppdatera utseendet på skärmen.

Till sist lägger vi våra resurser i Macroquads `storage` som är en global
persitent lagring. Det kan användas för att spara spelconfig som måste vara
tillgänglig överallt i spelkoden utan att behöva skicka runt datan.

```admonish info
Både `coroutines` och `storage` är experimentell funktionalitet i Macroquad och
användningen av dem kan komma att ändras i framtiden.
```

## Implementering 

### Importera

Det första vi ska göra är att importera `coroutines::start_coroutine` och
`collections::storage` från Macroquads experimentella namespace.

```rust
{{#include ../../mitt-spel/examples/coroutines-and-storage.rs:import}}
```

### Skapa ny load-metod

Nu kan vi skapa en ny metod `load()` i implenteringsblocket för structen
`Resources`. Där lägger vi in koden som sköter laddningen av resurserna med
en coroutine och uppdaterar skärmen med information om att resurserna laddas.

Funktionen `start_coroutine` tar ett `async` block och returnerar en
`Coroutine`. I async-blocket instantierar vi structen `Resources` som laddar
in alla resurser. Därefter använder vi `storage::store()` för att spara
resurserna i Macroquads storage. Det gör att vi kan komma åt resurserna från
andra platser i koden.

Med metoden `is_done()` på `Coroutine` kan vi kolla om coroutinen har kört
klart eller inte. Vi skapar en loop som kör tills `is_done()` returnerar
`true`. Medan coroutinen kör använder vi `draw_text()` för att skriva ut
en text på skärmen. Vi lägger också till 1 till 3 punkter efter texten med
hjälp av koden `".".repeat(((get_time() * 2.) as usize) % 4)`. Vi måste
också använda `clear_background()` och `next_frame.await` i loopen för att
uppdateringen ska fungera rätt.

```rust
{{#include ../../mitt-spel/examples/coroutines-and-storage.rs:load}}
```

```admonish info
Lite mer information om [Macroquads
coroutines](https://docs.rs/macroquad/latest/macroquad/experimental/coroutines/index.html)
och
[storage](https://docs.rs/macroquad/latest/macroquad/experimental/collections/storage/index.html)
finns att läsa i dokumentationen.
```

### Ladda resurserna

Anropet till att ladda resurserna måste uppdateras så att den använder den nya
`load()` metoden istället för att köra `new()` direkt. Eftersom `load()`
sparar resurserna i Macroquads storage så använder vi `storage::get::<Resources>()`
för att hämta resurserna och tilldela till variabeln `resources`.

```rust [hl,2-3]
{{#include ../../mitt-spel/examples/coroutines-and-storage.rs:loadresources}}
```

## Prova spelet

Spelet ska starta och medans filerna laddas ska meddelandet "Laddar
resurser..." visas mitt på skärmen. Troligen går det dock så fort att det
inte hinner synas när filerna läses direkt från disk. 

<div class="noprint no-page-break">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/coroutines-and-storage.rs:all}}
```
</details>
</div>


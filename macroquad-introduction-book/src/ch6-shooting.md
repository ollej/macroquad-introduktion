# Skjuta

Det känns lite orättvist att vår stackars cirkel inte kan försvara sig mot de
läskiga fyrkanterna. Därför är det dags att implementera skott som cirkeln kan
skjuta ner fyrkanterna med.

## Implementering

### Känner sig träffade

För att hålla reda på vilka fyrkanter som har blivit träffade av kulor så
lägger vi till ett nytt fält `collided` av typen `bool` i structen `Shape`.

```rust [hl,6]
{{#include ../../mitt-spel/examples/shooting.rs:shape}}
```

### Vektor för kulor

Vi måste ha en ny vektor som håller reda på alla kulor som har skjutits. Vi
kallar den `bullets` och skapar den efter vektorn med `squares`. Här anger vi
vilken typ vektorn ska innehålla eftersom Rust-kompilatorn måste veta vilken
typ det är innan vi har tilldelat den något värde. Vi använder structen
`Shape` även för kulorna för enkelhetens skull.

```rust
{{#include ../../mitt-spel/examples/shooting.rs:bullets}}
```

### Skjut kulor

Efter cirkeln har förflyttats så lägger vi till en kontroll om spelaren har
tryckt på mellanslag, och lägger till en kula i vektorn med kulor. Kulans x-
och y-koordinater sätts till samma som cirkeln, och hastigheten till dubbla
cirkelns hastighet.

```rust
{{#include ../../mitt-spel/examples/shooting.rs:shoot}}
```

Eftersom vi har lagt till ett fält på structen `Shape` måste vi lägga till den
när vi skapar en fyrkant.

```rust [hl,6]
{{#include ../../mitt-spel/examples/shooting.rs:squarecollided}}
```

### Ta bort kulor och fyrkanter

Även kulorna behöver tas bort om de hamnar utanför skärmen.

```rust
{{#include ../../mitt-spel/examples/shooting.rs:removebullets}}
```

Nu är det dags att ta bort alla fyrkanter och kulor som har kolliderat med
något. Det gör vi enkelt med `retain`-metoden och behåller alla som inte har
`collided` satt till `true`. Vi gör detta på båda vektorerna för `squares` och
`bullets`.

```rust
{{#include ../../mitt-spel/examples/shooting.rs:removecollided}}
```

### Kollidering

Efter vi har kollat om cirkeln har kolliderat med en fyrkant lägger vi till en
kontroll om någon fyrkant blir träffad av en kula. Både kulan och fyrkanten
uppdateras och fältet `collided` sätts till `true` så att vi kan ta bort dem
längre ned i koden.

```rust
{{#include ../../mitt-spel/examples/shooting.rs:collided}}
```

### Rensa kulor

Om det har blivit game over måste vi även rensa vektorn `bullets` så att
kulorna försvinner när ett nytt spel påbörjas.

```rust [hl,3]
{{#include ../../mitt-spel/examples/shooting.rs:clearbullets}}
```

### Rita ut kulor

Innan vi ritar ut cirkeln så ritar vi ut alla kulor, så att de ritas ut under
övriga former.

```rust
{{#include ../../mitt-spel/examples/shooting.rs:drawbullets}}
```

Det var allt för att kunna skjuta sönder fyrkanter.

```admonish tip
För att öka svårighetsgraden går det att lägga till en begränsning så att det
måste gå en viss tid mellan varje skott. Använd funktionen `get_time()` för
att spara undan när varje skott skjuts och jämför aktuella tiden med detta
värde.
```

## Kompletta källkoden

Källkoden för vårt spel ska nu se ut så här:

```rust
{{#include ../../mitt-spel/examples/shooting.rs:all}}
```

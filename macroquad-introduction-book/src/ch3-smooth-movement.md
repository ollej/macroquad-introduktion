## Mjukare rörelser

Eftersom Macroquad kommer rita bildrutor så snabbt som den kan måste vi kolla
hur lång tid som har gått mellan varje uppdatering för att avgöra hur långt
cirkeln ska förflyttas. Annars kommer vårt spel gå olika fort på olika
datorer, beroende på hur snabbt dom kan köra programmet.

Vi ska därför utöka programmet och lägga till en konstant variabel som avgör
hur snabbt cirkeln ska röra sig. Vi kallar den `MOVEMENT_SPEED` och börjar med
att tilldela den värden `50.0`. Går det för fort eller för sakta kan vi sänka
eller öka detta värde.

```rust
{{#include ../../mitt-spel/examples/smooth-movement.rs:constant}}
```

Därefter använder vi funktionen `get_frame_time()` som ger oss hur lång tid i
sekunder det har gått sedan föregående bildruta ritades på skärmen och
tilldelar den till variabeln `delta_time`.

```rust
{{#include ../../mitt-spel/examples/smooth-movement.rs:deltatime}}
```

Förändringen av variablerna `x` och `y` kan sedan bytas ut till en
multiplikation av värdena för `MOVEMENT_SPEED` och `delta_time` för att få hur
långt cirkeln ska förflyttas under denna bildruta.

```rust [hl,2,5,8,11]
{{#include ../../mitt-spel/examples/smooth-movement.rs:movement}}
```

Slutligen vill vi också att cirkeln aldrig ska hamna utanför fönstret, därför
begränsar vi variablerna `x` och `y`.

```rust
{{#include ../../mitt-spel/examples/smooth-movement.rs:clamp}}
```

```admonish info
Metoderna `min()` och `max()` används för att få det minsta eller högsta av
två värden.
```

```admonish tip
Ändra konstanten `MOVEMENT_SPEED` om cirkeln rör sig för fort eller för sakta.
Vad behöver ändras för att hela cirkeln ska vara kvar på skärmen?
```

### Källkod

Nu ser vårt program ut så här:

```rust
{{#include ../../mitt-spel/examples/smooth-movement.rs:all}}
```


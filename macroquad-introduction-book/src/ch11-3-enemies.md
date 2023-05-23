# Animerade fiender

Nu är det bara fienderna som behöver bytas från tråkiga fyrkanter till lite
mer spännande grafik. Det här fungerar likadant som med skeppet, vi laddar in
en textur, skapar en animeringssprite och byter hur fienderna ritas ut.

Ladda in texturen `enemy-small.png` och sätt filter mode till `Nearest`.

```rust [hl,1-4]
{{#include ../../mitt-spel/examples/graphics-enemies.rs:loadresources}}
```

Skapa en `AnimatedSprite` som beskriver vilka animationer som finns i
texturen. Det är bara en animering med två bildrutor. Grafiken för fienden är
16x16 bildrutor, men texturen har en pixels mellanrum mellan bildrutorna för
att inte orsaka blödning mellan rutorna när vi skalar texturen.

```rust
{{#include ../../mitt-spel/examples/graphics-enemies.rs:sprite}}
```

Även fiendens sprite måste uppdateras.

```rust
{{#include ../../mitt-spel/examples/graphics-enemies.rs:updatesprites}}
```

Nu kan vi byta ut utritningen av fyrkanter till att rita ut texturen från
animeringen. Vi hämtar ut bildrutan från `enemy_frame` och använder dess
`source_rect` i `DrawTextureParams`. Eftersom fienderna har slumpad storlek så
utgår vi från fiendens storlek när vi sätter `dest_size` och X- och
Y-koordinater.

```rust [hl,1,3-13]
{{#include ../../mitt-spel/examples/graphics-enemies.rs:drawenemy}}
```

Nu har vi bytt till grafik för alla element i spelet och det ser mer ut som
ett riktigt spel.

## Kompletta källkoden

Källkoden för vårt spel ska nu se ut så här:

```rust
{{#include ../../mitt-spel/examples/graphics-enemies.rs:all}}
```


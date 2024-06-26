# Animerade fiender

![Screenshot](images/graphics-enemies.gif#center)

Nu är det bara fienderna som behöver bytas från tråkiga fyrkanter till lite
mer spännande grafik. Det här fungerar likadant som med skeppet, vi laddar in
en textur, skapar en animeringssprite och byter hur fienderna ritas ut.

```admonish bug
I version 0.4.4 av Macroquad är det en bugg som gör att texturerna inte
fungerar som de ska när `build_textures_atlas` används. Om texturerna ser
konstiga ut eller flimrar så prova att ta bort detta anrop.
```

## Implementering

### Ladda in textur

Ladda in texturen `enemy-small.png` och sätt filter mode till `Nearest`.

```rust [hl,1-4]
{{#include ../../my-game/examples/graphics-enemies.rs:loadresources}}
```

### Skapa animering

![Spritesheet för små fiender](assets/enemy-small.png#pixelated)

Skapa en `AnimatedSprite` som beskriver vilka animationer som finns i
texturen. Det är bara en animering med två bildrutor. Grafiken för fienden är
16x16 pixlar, men texturen har en pixels mellanrum mellan bildrutorna för
att inte orsaka blödning mellan rutorna när vi skalar texturen.

```rust
{{#include ../../my-game/examples/graphics-enemies.rs:sprite}}
```

### Uppdatera animering

Även fiendens sprite måste uppdateras efter animeringarna för rymdskeppet och
kulorna.

```rust [hl,3]
{{#include ../../my-game/examples/graphics-enemies.rs:updatesprites}}
```

### Rita bildrutor för fiender

Nu kan vi byta ut utritningen av fyrkanter till att rita ut texturen från
animeringen. Vi hämtar ut bildrutan från `enemy_small_sprite` och använder dess
`source_rect` i `DrawTextureParams`. Eftersom fienderna har slumpad storlek så
utgår vi från fiendens storlek när vi sätter `dest_size` och X- och
Y-koordinater.

```rust [hl,1,3-13]
{{#include ../../my-game/examples/graphics-enemies.rs:drawenemy}}
```

Nu har vi bytt till grafik för alla element i spelet och det ser mer ut som
ett riktigt spel.

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../my-game/examples/graphics-enemies.rs:all}}
```
</details>
</div>


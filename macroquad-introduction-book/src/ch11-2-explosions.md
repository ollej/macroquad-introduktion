# Grafiska explosioner

![Screenshot](images/graphics-explosions.gif#center)

För att göra explosionerna lite mer effektfulla så ska vi lägga till grafik
även för partiklarna.

```admonish bug
I version 0.4.4 av Macroquad är det en bugg som gör att texturerna inte
fungerar som de ska när `build_textures_atlas` används. Om texturerna ser
konstiga ut eller flimrar så prova att ta bort detta anrop.
```

## Implementering

### Importering

Vi börjar med att uppdatera importeringen från paketet `macroquad_particles`,
och byta ut `ColorCurve` mot `AtlasConfig`.

```rust
{{#include ../../my-game/examples/graphics-explosion.rs:import}}
```

### Uppdatera partikelkonfigurationen

Nu behöver vi uppdatera konfigurationen för vår `particle_explosion` så att
den använder en `AtlasConfig` som beskriver hur den ska rita partiklarna från
en textur istället för att använda `ColorCurve`. Vi uppdaterar även storleken
och livstiden för att passa bättre med grafiken.

```rust [hl,10,12,14]
{{#include ../../my-game/examples/graphics-explosion.rs:emitterconfig}}
```

### Ladda in texturer

![Spritesheet för explosionen](assets/explosion.png#pixelated)

Innan vi bygger texturatlasen så laddar vi in texturen med animeringen för
partiklarna. Filen med animeringen heter `explosion.png`. Glöm inte att sätta
filtret till `FilterMode::Nearest`.

```rust [hl,1-4]
{{#include ../../my-game/examples/graphics-explosion.rs:loadresources}}
```

### Lägg till texturen

När vi skapar explosionen måste vi lägga till texturen, och vi uppdaterar även
mängden för att få lite fler partiklar. Här måste vi anropa metoden `clone()`
på texturen, vilket går väldigt snabbt då det bara är en pekare till texturen.

```rust [hl,3-4]
{{#include ../../my-game/examples/graphics-explosion.rs:explosiontexture}}
```

När du kör spelet nu ska explosionerna animeras med hjälp av
explosionstexturen istället för att vara flerfärgade rutor.

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../my-game/examples/graphics-explosion.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/explosions.toml}}

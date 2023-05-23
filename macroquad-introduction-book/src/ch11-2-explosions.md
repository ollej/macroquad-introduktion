# Grafiska explosioner

För att göra explosionerna lite mer effektfulla så ska vi lägga till grafik
även för partiklarna.

Vi börjar med att uppdatera importeringen från paketet `macroquad_particles`,
och byta ut `ColorCurve` mot `AtlasConfig`.

```rust
{{#include ../../mitt-spel/examples/graphics-explosion.rs:import}}
```

Nu behöver vi uppdatera konfigurationen för vår `particle_explosion` så att
den använder en `AtlasConfig` som beskriver hur den ska rita partiklarna från
en textur istället för att använda `ColorCurve`. Vi uppdaterar även storleken
och livstiden för att passa bättre med grafiken.

```rust [hl,10,12,14]
{{#include ../../mitt-spel/examples/graphics-explosion.rs:emitterconfig}}
```

Innan vi bygger texturatlasen så laddar vi in texturen med animeringen för
partiklarna. Filen med animeringen heter `explosion.png`. Glöm inte att sätta
filtret till `FilterMode::Nearest`.

```rust [hl,1-4]
{{#include ../../mitt-spel/examples/graphics-explosion.rs:loadresources}}
```

När vi skapar explosionen måste vi lägga till texturen, och vi uppdaterar även
mängden för att få lite fler partiklar.

```rust [hl,3-4]
{{#include ../../mitt-spel/examples/graphics-explosion.rs:explosiontexture}}
```

När du kör spelet nu ska explosionerna animeras med hjälp av
explosionstexturen istället för att vara flerfärgade rutor.

## Kompletta källkoden

Källkoden för vårt spel ska nu se ut så här:

```rust
{{#include ../../mitt-spel/examples/graphics-explosion.rs:all}}
```


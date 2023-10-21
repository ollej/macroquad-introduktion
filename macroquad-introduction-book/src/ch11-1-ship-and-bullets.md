# Rymdskepp och kulor

![Screenshot](images/graphics-spaceship.gif#center)

Först ska vi lägga till grafik för rymdskeppet som spelaren styr. Den kommer
att animeras med två olika sprites, och kommer även ha olika animeringar för
om skeppet styr åt höger eller vänster. Dessutom lägger vi till en textur med
animering för kulorna som skeppet skjuter.

## Implementering

### Importera

Då animeringsstödet i Macroquad fortfarande räknas som experimentell måste vi
importera stödet för det explicit längst upp i källkodsfilen. Det är
structarna `AnimatedSprite` och `Animation` vi kommer använda oss av.

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:import}}
```

### Konfigurera assets-katalog

Först måste vi ange var Macroquad ska läsa resurserna ifrån, därför använder
vi funktionen `set_pc_assets_folder()` som tar sökvägen till
`assets`-katalogen med utgång från spelets rotkatalog. Detta behövs för att
olika plattformar placerar filerna på olika ställen, och vi slipper dessutom
ange katalogen för varje fil som ska laddas in. Lägg in nedanstående kod i
`main`-funktionen innan loopen. 

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:assetsfolder}}
```

### Ladda in texturer

Nu kan vi ladda in filerna med texturerna för animeringarna av skeppet och
kulorna. För att ladda in en textur används funktionen `load_texture()` som
tar namnet på filen. Det är en asynkron funktion, så vi måste köra `await` och
vänta på att inladdningen är klar. Filladdningen kan misslyckas, så vi får
tillbaka ett `Result` så vi använder oss av `expect()` för att avsluta
programmet med ett felmmeddelande om det uppstår. 

Efter att texturen är inladdad så sätter vi vilket sorts filter som ska
användas när texturen skalas upp med metoden `set_filter()`. Vi sätter
`FilterMode::Nearest` för att vi vill bibehålla pixelutseendet. Det här måste
vi göra på varje textur.

Vi laddar in filerna `ship.png` som innehåller animeringarna för skeppet, och
`laser-bolts.png` som innehåller animeringar för två olika sorts kulor.

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:loadresources}}
```

```admonish info
Bilderna returneras som structen `Texture2D` som innehåller bilddatan som
sparas i GPU-minnet. Motsvarande struct för bilder som sparas i CPU-minnet är
`Image`.
```

### Bygg en texturatlas

Efter att alla texturer har laddats in anropar vi Macroquad-funktionen
`build_textures_atlas` som bygger upp en atlas som innehåller alla inladdade
texturer. Det gör att alla anrop till `draw_texture()` kommer använda texturen
från atlasen istället för varje separat textur. Alla texturer bör laddas in
innan detta anrop.

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:atlas}}
```

### Animering av rymdskeppet

![Spritesheet för rymdskeppet](assets/ship.png#pixelated)

Nu måste vi beskriva hur animeringarna i texturerna ska visas. Det gör vi
genom att skapa en `AnimatedSprite` för varje textur. Storleken på varje
bildruta i skeppets textur är 16x24 pixlar, därför sätter vi `tile_width` till
`16` och `tile_height` till `24`.

Därefter kommer en array som beskriver alla animeringar som ingår i texturen.
Varje animation i en textur ligger på varsin rad, med bildrutorna efter
varandra åt höger. Varje `Animation` ska ha ett beskrivande namn, vilken rad
i texturen som innehåller animationen, hur många bildrutor det är samt hur
många bildrutor som ska visas per sekund.

Skeppet har tre animationer, den första är när den flyger rakt upp eller ner,
den andra när det åker åt vänster och den tredje när det åker åt höger. Det är
två bildrutor per animation och dom ska visas med 12 bildrutor per sekund.
Texturen innehåller två animationer till, på rad 1 och 3 som visar skeppet
lite mindre vinklade svängar.

Avslutningsvis sätter vi `playing` till `true` för att vi vill att animeringen
ska vara aktiv.

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:shipsprite}}
```

### Animering av kulor

![Spritesheet för rymdskeppet](assets/laser-bolts.png#pixelated)

Animeringen för kulorna är väldigt lika, det är två animeringar med två
bildrutor var som ska visas med 12 bildrutor per sekund. Storleken på bilderna
är 16x16 pixlar. Vi kommer bara använda den andra animeringen, så vi använder
oss av metoden `set_animation()` för att sätta att det är animeringen på rad
`1` som ska användas.

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:bulletsprite}}
```

### Animera riktning

För skeppet behöver vi sätta vilken animation som ska användas baserat på åt
vilket håll skeppet åker. I koden som sätter vilket håll skeppet ska
förflyttas behöver vi därför köra metoden `set_animation` på vår
`ship_sprite`. Vi sätter först animationen `0` som inte svänger åt något håll,
om skeppet ska förflyttas åt höger sätter vi animeringen till `2` och om den
förflyttas åt höger sätter vi `1`.

```rust [hl,1,5,10]
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:updateship}}
```

### Ändra kulstorlek

Eftersom grafiken för kulorna är större än den lilla cirkeln vi ritade ut
tidigare måste vi uppdatera storleken och startpositionen när vi skapar kulorna.

```rust [hl,4,6]
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:shoot}}
```

### Uppdatera animeringar

För att Macroquad ska kunna animera texturerna åt oss måste vi anropa metoden
`update()` på varje sprite inne i loopen. Vi lägger därför till följande två
rader nedanför koden som uppdaterar fienders och kulors position.

```rust [hl,8-9]
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:updatesprites}}
```

### Rita kulornas bildrutor

Nu kan vi använda oss av funktionen `draw_texture_ex()` för att rita ut
rätt bildruta från animeringen. Vi byter ut raderna som ritar ut en cirkel för
varje kula till följande rader. Först anropar vi `frame()` på `bullet_sprite`
för att få ut aktuell bildruta och tilldelar den till variabeln
`bullet_frame`.

Inne i loopen som ritar ut alla kulor anropar vi `draw_texture_ex()` för att
rita ut kulan. Den tar `bullet_texture` som argument, därefter en X och
Y-position som vi räknar ut baserat på kulans storlek. Vi skickar även med
structen `DrawTextureParams` med värdena `dest_size` och `source_rect`. Fältet
`dest_size` avgör hur stort texturen ska ritas ut, så vi skickar in en `Vec2`
med kulans storlek för både X och Y. Därefter anropar vi
`bullet_frame.source_rect` som anger var i texturen aktuell bildruta ska
hämtas.

```rust [hl,1,3-12]
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:drawbullets}}
```

```admonish info
Med hjälp av
[`DrawTextureParams`](https://docs.rs/macroquad/0.3.25/macroquad/texture/struct.DrawTextureParams.html)
går det att ändra hur texturen ska ritas ut.  Det går att rita ut texturen
roterat eller spegelvänt med fälten `rotation`, `pivot`, `flip_x` och `flip_y`. 
```

### Rita ut rymdskeppets bildrutor

Till sist kan vi byta ut cirkeln mot texturen för skeppet. Det fungerar
likadant som för att rita ut kulorna. Vi hämtar först ut aktuell bildruta från
spritens animering och ritar sedan ut texturen med `draw_texture_ex`.

Då skeppanimeringen inte har samma storlek i höjd och bredd så använder vi oss
av `ship_frame.dest_size` för att få ut storleken den ska ritas ut i. Men för
att det inte ska bli så smått ritar vi ut den med dubbla storleken.

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:drawship}}
```

Om allt fungerar som det ska så ska det nu vara grafik för både skeppet och
kulorna.

```admonish tip title="Utmaning" class="challenge"
Prova att använda de två extra skeppanimationerna för att vinkla skeppet lite
mindre precis när det bytt håll för att sedan vinklas fullt ut efter en viss
tid.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/graphics-spaceship.rs:all}}
```
</details>
</div>


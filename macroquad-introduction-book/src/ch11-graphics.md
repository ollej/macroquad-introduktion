# Grafik

Nu börjar det bli dags att lägga till grafik i vårt spel så det börjar se ut
som ett riktigt spel. Vi kommer göra det i tre omgångar, för att det inte ska
bli för mycket ändringar på en gång. Till en början kommer vi lägga in
inladdningen av texturer direkt i vår main-funktion och byta ut
ritnings-funktionerna i huvudloopen. I ett senare kaptitel kommer vi titta på
att bryta ut det till separata delar.

Innan vi ändrar någon kod behöver vi ladda ner alla resurser som behövs. Ladda
ner det här [paketet med grafik och ljud](assets.zip) och packa upp det och
lägg filerna i en katalog som heter `assets` i rotkatalogen för ditt spel.
Alla resurser är public domain och har framförallt hämtats från webbplatsen
[OpenGameArt.org](https://opengameart.org/) där det finns alla möjliga
resurser för att skapa spel. 

Filstrukturen för ditt spel bör nu se ut såhär:

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── assets
│   ├── 8bit-spaceshooter.ogg
│   ├── atari_games.ttf
│   ├── button_background.png
│   ├── button_clicked_background.png
│   ├── enemy-big.png
│   ├── enemy-medium.png
│   ├── enemy-small.png
│   ├── explosion.png
│   ├── explosion.wav
│   ├── laser-bolts.png
│   ├── laser.wav
│   ├── ship.png
│   └── window_background.png
└── src
    ├── main.rs
    └── starfield-shader.glsl
```

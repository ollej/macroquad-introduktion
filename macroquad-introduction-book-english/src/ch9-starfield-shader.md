# Stjärnfält med en shader

![Screenshot](images/starfield-shader.gif#center)

Den gröna bakgrunden börjar kännas lite tråkig, så nu är det dags att göra en
lite mer intressant bakgrund. Vi kommer använda oss av en pixel shader för att
göra ett stjärnfält. Hur man kodar en shader ligger utanför den här guiden,
utan vi kommer använda oss av en färdig utan att gå in på detaljerna.

Kortfattat är en shader ett program som körs på datorns GPU, skrivet i ett
C-liknande programmeringsspråk som kallas GLSL. Shadern består av två delar,
en vertex shader och en fragment shader. Vertex shadern konverterar från
koordinater i en 3D-miljö till 2D-koordinater för en skärm. Fragment shadern
körs sedan för varje pixel på skärmen, och sätter variabeln `gl_FragColor` som
avgör vilken färg pixeln ska ha. Eftersom vårt spel är i 2D så gör vertex
shadern ingenting mer än att sätta positionen.

## Implementering

### Shaders

Längst upp i `main.rs` ska vi lägga till en vertex shader och fragment-shadern
från en fil som vi kommer skapa senare. Vi använder oss av Rusts macro
`include_str!()` som läser in filen som en `&str` vid kompileringen.
Vertex-shadern är så kort att den kan läggas in direkt här i källkoden.

Den viktigaste raden i shadern är den som sätter `gl_Position`. För
enkelhetens skull sätter vi `iTime` som används av shadern från `_Time.x`. Det
hade också gått att använda `_Time` direkt i shadern.

```rust
{{#include ../../mitt-spel/examples/starfield-shader.rs:shaders}}
```

### Initialisera shadern

I vår `main()` funktion, innan loopen, så måste vi sätta upp några variabler
för att kunna rita ut shadern. Vi börjar med att skapa variabeln
`direction_modifier` som vi ska använda för att påverka hur stjärnorna rör sig
medan cirkeln förflyttas i sidled. Därefter skapar vi en `render_target` som
shadern kommer att renderas till.

Sen laddar vi in vår vertex shader och fragment shader till en `Material` med
hjälp av en `ShaderSource::Glsl`. 

I parametrarna sätter vi även upp två uniforms till shadern som är globala
variabler som vi kan sätta för varje bildruta. Uniformen `iResolution`
innehåller fönstrets storlek, och `direction_modifier` kommer användas för att
styra åt vilken riktning stjärnorna ska röra sig.

```rust
{{#include ../../mitt-spel/examples/starfield-shader.rs:setupshader}}
```

```admonish info
Macroquad lägger automatiskt in några uniforms till shaders. De som finns
tillgängliga är `_Time`, `Model`, `Projection`, `Texture` och
`_ScreenTexture`.
```

### Rita ut shadern

Nu är det dags att byta ut den lila bakgrund till vårt stjärnfält. Byt ut
kodraden `clear_background(DARKPURPLE);` till nedanstående kod.

Först måste vi tilldela fönstrets upplösning till materialets uniform
`iResolution` för att alltid få rätt fönsterstorlek. Vi sätter även uniformen
`direction_modifier` till värdet av den motsvarande variabeln.

Därefter använder vi funktionen `gl_use_material()` för att använda
materialet. Slutligen använder vi funktionen `draw_texture_ex()` för att rita
ut texturen från vår `render_target` på skärmens bakgrund. Innan vi fortsätter
återställer vi shadern med `gl_use_default_material()` så den inte används när
vi ritar ut resten av spelet.

```rust
{{#include ../../mitt-spel/examples/starfield-shader.rs:drawshader}}
```

### Styr stjärnornas rörelse

När spelaren håller ner höger eller vänster piltangent så lägger vi till
eller drar ifrån ett värde från variabeln `direction_modifier` så att shadern
kan ändra riktningen på stjärnornas rörelse. Även här multiplicerar vi värdet
med `delta_time` så det blir relativt till hur lång tid det har tagit sedan
föregående bildruta.

 ```rust [hl,3,7]
{{#include ../../mitt-spel/examples/starfield-shader.rs:shaderdir}}
```

### Skapa fil för shadern

Till sist måste vi skapa en fil som innehåller fragment shadern. Skapa en fil
med namnet `starfield-shader.glsl` i din `src`-katalog och lägg in följande
kod:

```glsl
{{#include ../../mitt-spel/examples/starfield-shader.glsl}}
```

```admonish info
Om du vill veta hur shadern fungerar så kan du titta på videon [Shader Coding:
Making a starfield](https://youtu.be/rvDo9LvfoVE) av The Art of Code.
```

Nu är vårt stjärnfält klart och vårt spel börjar se ut som det utspelar sig i
rymden!

```admonish tip title="Utmaning" class="challenge"
Titta på videon som nämns ovan och se om du kan ändra på färger och storlek
på stjärnorna.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/starfield-shader.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/starfield-shader.toml}}

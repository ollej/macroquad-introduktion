# Fallande fyrkanter

![Screenshot](images/falling-squares.gif#center)

För att det ska hända lite mer i vårt spel är det dags att skapa lite
action. Eftersom hjälten i vårt spel är en modig cirkel så får våra
motståndare bli kantiga fyrkanter som faller ner från toppen av fönstret.

## Implementering

### Struct för former

För att hålla reda på vår cirkel och alla fyrkanter så skapar vi en struct som
vi kan ge namnet `Shape` som innehåller storlek, hastighet samt x och
y-koordinater.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:shape}}
```

### Initiera slumpgenerator

Vi kommer använda oss av en slumpgenerator för att avgöra när nya fyrkanter
ska komma in på skärmen. Därför behöver vi seeda slumpgeneratorn så att det
inte blir samma slumptal varje gång. Detta görs i början av `main`-funktionen
med metoden `rand::srand()` som vi skickar in nuvarande tid till som seed.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:srand}}
```

```admonish note title="Notera"
Vi använder oss av metoden `miniquad::date::now()` från det underliggande
[grafikramverket Miniquad](https://docs.rs/miniquad/latest/miniquad/index.html)
för att få den aktuella tiden.
```

### Vektor med fyrkanter

I början av `main`-funktionen skapar vi en vektor `squares` som kommer
innehålla alla fyrkanter som ska visas på skärmen. Den nya variabeln `circle`
får representera vår hjälte, den fantastiska cirkeln. Hastigheten använder
konstanten `MOVEMENT_SPEED` och `x` och `y`-fälten sätts till mitten av
skärmen.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:variables}}
```

### Skapa nya fyrkanter

Nu är det dags att starta invasionen av fyrkanter. Här delar vi som tidigare
upp förflyttningen och utritningen av fyrkanterna. Det gör att förflyttningen
inte behöver vara beroende av uppdateringsfrekvensen av skärmen, och vi kan se
till att alla förändringar har skett innan vi börjar rita upp något på
skärmen.

Först använder vi oss av funktionen `rand::gen_range()` för att avgöra om vi ska
lägga till en ny fyrkant. Den tar två argument, ett lägsta värde och ett
högsta värde, och returnerar sedan ett slumpat tal mellan dom två värdena. Om
värdet är tillräckligt högt så skapar vi en ny Shape och lägger till i vektorn
`squares`. För att få lite variation använder vi även `rand::gen_range()` för
att få olika storlek, hastighet och startposition på alla fyrkanter.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:generatesquare}}
```

```admonish note title="Notera"
Rektanglar ritas ut med början från övre vänstra hörnet. Därför subtraherar vi
halva fyrkantens storlek när vi räknar ut X-positionen. Y-positionen börjar på
negativt av fyrkantens storlek, så att den börjar helt utanför skärmen.
```

### Uppdatera fyrkanters position

Nu kan vi gå igenom hela vektorn med en for-loop och uppdatera y-positionen
med hjälp av fyrkantens hastighet och variabeln `delta_time`. Detta gör att
fyrkanterna kommer åka neråt över skärmen.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:movesquares}}
```

### Rensa bort fyrkanter som inte syns

Därefter måste vi rensa upp alla fyrkanter som har hamnat utanför skärmen då
det är onödigt att rita ut saker som inte syns. Vi använder oss av metoden
`retain()` på vektorn som tar en funktion som avgör om elementen ska behållas.
Vi kollar att fyrkantens y-värde fortfarande är mindre än höjden på fönstret
plus storleken på fyrkanten.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:removesquares}}
```

### Rita ut fyrkanterna

Till sist lägger vi till en for-loop som går igenom vektorn `squares` och
använder funktionen `draw_rectangle()` för att rita ut en rektangel på den
uppdaterade positionen och med rätt storlek. Eftersom rektanglar ritas ut med
x och y från hörnet längst upp till vänster och våra koordinater utgår från
center av fyrkanten så använder vi lite matematik för att räkna ut var dom ska
placeras. Storleken används två gånger, en gång för fyrkantens bredd och en
gång för fyrkantens höjd. Vi sätter färgen till `GREEN` så att alla fyrkanter
blir gröna.

```admonish note title="Notera"
Det finns även funktionen
[`draw_rectangle_ex()`](https://docs.rs/macroquad/latest/macroquad/shapes/fn.draw_rectangle_ex.html)
som tar structen
[`DrawTextureParams`](https://docs.rs/macroquad/latest/macroquad/shapes/struct.DrawRectangleParams.html)
istället för en färg. Med den kan man förutom färg även sätta `rotation`
och `offset` på rektangeln.
```

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:drawsquares}}
```

```admonish tip title="Utmaning" class="challenge"
Försök att ge olika färger till fyrkanterna genom att använda metoden
`choose()` på vektorer från Macroquads 
[ChooseRandom trait](https://docs.rs/macroquad/latest/macroquad/rand/trait.ChooseRandom.html)
som returnerar ett slumpmässigt valt element från vektorn.
```

<div class="noprint">

## Komplett källkod

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/falling-squares.toml}}

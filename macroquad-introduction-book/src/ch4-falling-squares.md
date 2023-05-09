# Fallande fyrkanter

För att det ska hända lite mer i vårt spel är det dags att skapa lite
action. Eftersom hjälten i vårt spel är en modig cirkel så får våra
motståndare bli kantiga fyrkanter som faller ner från toppen av fönstret.

För att hålla reda på vår cirkel och alla fyrkanter så skapar vi en struct som
heter `Shape` som innehåller storlek, hastighet samt x och y-koordinater.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:shape}}
```

Vi kommer använda oss av en slumpgenerator för att avgöra när nya fyrkanter
ska komma in på skärmen. Därför behöver vi seeda slumpgeneratorn så att det
inte blir samma slumptal varje gång. Detta görs i början av `main`-funktionen
med metoden `srand()` som vi skickar in nuvarande tid till som seed. Eftersom
rand-funktionerna inte är med i Macroquads "prelude"-modul måste vi även
importera den längst upp i koden.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:srand}}
```

I början av `main`-funktionen skapar vi en vektor `squares` som kommer
innehåll alla fyrkanter som ska visas på skärmen. Den nya variabeln `circle`
får representera vår hjälte, den fantastiska cirkeln. Hastigheten använder
konstanten `MOVEMENT_SPEED` och `x` och `y`-fälten sätts till mitten av
skärmen.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:variables}}
```

Nu är det dags att starta invasionen av fyrkanter. Här delar vi som tidigare
upp förflyttningen och utritningen av fyrkanterna. Det gör att förflyttningen
inte behöver vara beroende av uppdateringsfrekvensen av skärmen, och vi kan se
till att alla förändringar har skett innan vi börjar rita upp något på
skärmen.

Först använder vi oss av funktionen `gen_range` för att avgöra om vi ska lägga
till en ny fyrkant. Den tar två argument, ett lägsta värde och ett högsta
värde, och returnerar sedan ett slumpat tal mellan dom två värdena. Om värdet
är tillräckligt högt så skapar vi en ny Shape och lägger till i vektorn
`squares`. För att få lite variation använder vi även `gen_range()` för att få
olika storlek, hastighet och startposition på alla fyrkanter.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:generatesquare}}
```

Nu kan vi gå igenom hela vektorn med en for-loop och uppdatera y-positionen
med hjälp av fyrkantens hastighet och variabeln `delta_time`. Detta gör att
fyrkanterna kommer åka neråt över skärmen.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:movesquares}}
```

Därefter måste vi rensa upp alla fyrkanter som har hamnat utanför skärmen då
det är onödigt att rita ut saker som inte syns. Vi använder oss av metoden
`retain()` på vektorn som tar en funktion som avgör om elementen ska behållas.
Vi kollar att fyrkantens y-värde fortfarande är mindre än höjden på fönstret +
storleken på fyrkanten.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:removesquares}}
```

Till sist lägger vi till en for-loop som går igenom vektorn `squares` och
använder funktionen `draw_rectangle()` för att rita ut en rektangel på den
uppdaterade positionen och med rätt storlek. Eftersom rektanglar ritas ut med
x och y från hörnet längst upp till vänster och våra koordinater utgår från
center av fyrkanten så använder vi lite matematik för att räkna ut var dom ska
placeras.

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:drawsquares}}
```

## Komplett källkod

Hela programmet ser nu ut så här:

```rust
{{#include ../../mitt-spel/examples/falling-squares.rs:all}}
```



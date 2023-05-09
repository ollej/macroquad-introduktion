# Kollision

Våra ovänner fyrkanterna är ännu inte så farliga, så får att öka spänningen är
det dags att skapa konflikt. Om vår vän cirkeln kolliderar med en fyrkant så
är spelet över och måste startas om.

Efter att vi har ritat upp alla cirklar och fyrkanter så lägger vi till en
kontroll som ser om någon fyrkant rör vid cirkeln. Om den gör det så visar vi
texten Game Over och väntar på att spelaren trycker på space-tangenten. När
spelaren trycker på space så nollställs vektorn med fyrkanter och cirkeln
flyttas tillbaka till mitten av skärmen.

Vi utökar structen `Shape` med en implementation som innehåller metoden
`collides_with()` som kollar om den kolliderar med en annan `Shape`. Denna
använder sig av Macroquads `Rect` struct som har hjälpmetoden `overlaps()`. Vi
skapar även en egen hjälpmetod som skapar en `Rect` från vår `Shape`.

```rust
{{#include ../../mitt-spel/examples/collision.rs:implshape}}
```

I början av huvudloopen lägger vi till vår kontroll av kollisioner. Vi
använder metoden `any()` på vektorn `squares` och kollar om någon fyrkant
kolliderar med vår hjälte cirkeln. Om det har skett en kollision så sätter
vi variabeln `gameover` till true.

```rust
{{#include ../../mitt-spel/examples/collision.rs:collision}}
```

Om `gameover`-variabeln är sann och spelaren trycker på mellanslagstangenten
så tömmer vi vektorn `squares` med metoden `clear()` och återställer cirkelns
x och y-koordinater till mitten av skärmen.

```rust
{{#include ../../mitt-spel/examples/collision.rs:gameover}}
```

För att cirkeln och fyrkanterna inte ska röra sig medan det är game over så
görs all kod för förflyttning enbart om variabeln `gameover` är falsk.

```rust
        if !gameover {
            ...
        }
```

Slutligen ritar vi ut texten "Game Over!" i mitten av skärmen efter cirkeln
och fyrkanterna har ritats ut, men bara om variabeln `gameover` är sann.

```rust
{{#include ../../mitt-spel/examples/collision.rs:drawgameover}}
```

## Kompletta källkoden

Källkoden för vårt spel ska nu se ut så här:

```rust
{{#include ../../mitt-spel/examples/collision.rs:all}}
```



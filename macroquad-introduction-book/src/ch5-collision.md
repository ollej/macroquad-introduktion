# Kollisionskurs

![Screenshot](images/collision.gif#center)

Våra ovänner fyrkanterna är ännu inte så farliga, så får att öka spänningen är
det dags att skapa konflikt. Om vår vän cirkeln kolliderar med en fyrkant så
är spelet över och måste startas om.

Efter att vi har ritat upp alla cirklar och fyrkanter så lägger vi till en
kontroll som ser om någon fyrkant rör vid cirkeln. Om den gör det så visar vi
texten Game Over och väntar på att spelaren trycker på space-tangenten. När
spelaren trycker på space så nollställs vektorn med fyrkanter och cirkeln
flyttas tillbaka till mitten av skärmen.

## Implementering

### Kollisionsmetod

Vi utökar structen `Shape` med en implementation som innehåller metoden
`collides_with()` som kollar om den kolliderar med en annan `Shape`. Denna
använder sig av Macroquads
[`Rect`](https://docs.rs/macroquad/latest/macroquad/math/struct.Rect.html)
struct som har hjälpmetoden `overlaps()`. Vi skapar även en egen hjälpmetod
`rect()` som skapar en `Rect` från vår `Shape`.

```admonish info
Det finns många hjälpmetoder på `Rect` för göra beräkningar på rektanglar, som
`contains()`, `intersect()`, `scale()`, `combine_with()` och `move_to()`.
```

```rust
{{#include ../../my-game/examples/collision.rs:implshape}}
```

```admonish note title="Notera"
Macroquads `Rect` utgår också från övre vänstra hörnet, därför måste vi även
här subtrahera halva storleken från både X och Y.
```

### Är det game over?

Vi behöver en ny boolesk variabel `gameover` som håller reda på om spelaren
har dött som vi lägger in före huvudloopen.

```rust
{{#include ../../my-game/examples/collision.rs:variable}}
```

För att cirkeln och fyrkanterna inte ska röra sig medan det är game over så
görs all kod för förflyttning enbart om variabeln `gameover` är `false`.

```rust
        if !gameover {
            ...
        }
```

### Kollidering

Efter förflyttningskoden lägger vi till en kontroll om någon fyrkant
kolliderar med cirkeln. Vi använder metoden `any()` på iteratorn för vektorn
`squares` och kollar om någon fyrkant kolliderar med vår hjälte cirkeln. Om
det har skett en kollision sätter vi variabeln `gameover` till `true`.

```rust
{{#include ../../my-game/examples/collision.rs:collision}}
```

```admonish tip title="Utmaning" class="challenge"
Kollisionskoden utgår från att cirkeln är en fyrkant. Prova att skriva kod som
tar hänsyn till att cirkeln inte fyller ut hela fyrkanten.
```

### Återställning

Om `gameover`-variabeln är `true` och spelaren precis har tryckt på
mellanslagstangenten så tömmer vi vektorn `squares` med metoden `clear()` och
återställer cirkelns `x` och `y`-koordinater till mitten av skärmen. Sen
sätter vi variabeln `gameover` till `false` så att spelet kan börja igen.

```rust
{{#include ../../my-game/examples/collision.rs:gameover}}
```

```admonish info
Skillnaden mellan `is_key_down()` och `is_key_pressed()` är att den senare
bara kollar om tangenten trycktes ned under den aktuella bildrutan, medan den
tidigare returnerar sant för alla bildrutor från att knappen trycktes ned
och sedan hålls nedtryckt. Ett experiment du kan göra är att använda
`is_key_pressed()` för att styra cirkeln.

Det finns även `is_key_released()` som kollar om tangenten släpptes under den
aktuella bildrutan.
```

### Skriv ut Game Over

Slutligen ritar vi ut texten "Game Over!" i mitten av skärmen efter cirkeln
och fyrkanterna har ritats ut, men bara om variabeln `gameover` är `true`.

```admonish info
Det går också att använda funktionen
[`draw_text_ex()`](https://docs.rs/macroquad/latest/macroquad/text/fn.draw_text_ex.html)
som tar en [`DrawTextParams` struct](https://docs.rs/macroquad/latest/macroquad/text/struct.TextParams.html)
istället för `font_size` och `color`. Med den kan man ange fler parameterar som
`font`, `font_scale`, `font_scale_aspect` och `rotation`.
```

```rust
{{#include ../../my-game/examples/collision.rs:drawgameover}}
```

```admonish tip title="Utmaning" class="challenge"
Eftersom `draw_text()` utgår från textens baslinje så kommer texten inte visas
exakt i mitten av skärmen. Prova att använda fälten `offset_y` och `height`
från `text_dimensions` för att räkna ut textens mittpunkt. Macroquads exempel [text
measures](https://github.com/not-fl3/macroquad/blob/master/examples/text_measures.rs)
kan ge tips till hur det fungerar.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../my-game/examples/collision.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/collision.toml}}

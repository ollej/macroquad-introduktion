# Far å flyg

![Screenshot](images/move-a-circle.gif#center)

Ett spel är inte så roligt utan att det händer något på skärmen. Till att
börja med visar vi en boll som vi kan styra med knapptryckningar.

## Implementering

De första två raderna i main-funktionen använder funktionerna `screen_width()`
och `screen_height()` för att få bredden och höjden på fönstret. Dessa värden
delas med 2 för att få koordinaterna till mitten av skärmen, och tilldelas
till variablerna `x` och `y`.

```rust
{{#include ../../my-game/examples/move-a-circle.rs:coordinates}}
```

### Hantera tangenbordsinput

Inne i loopen rensar vi fortfarande skärmen, vilket måste göras vid varje
bildruta. Därefter kommer fyra if-satser som kollar om piltangerna är
nedtryckta och ändrar på variablerna `x` eller `y` som avgör var cirkeln ska
visas. Funktionen `is_key_down()` returnerar `true` om den angivna tangenten är
nedtryckt. Dess argument är enumen `KeyCode` som innehåller alla tangenter som
finns på ett tangentbord.

```rust
{{#include ../../my-game/examples/move-a-circle.rs:movement}}
```

```admonish info
Vilka andra tangenter som finns tillgängliga finns beskrivet i
[dokumentationen för KeyCode](https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html).
```

### Rita en cirkel

Slutligen ritas cirkeln ut på de angivna koordinaterna med en radie på 16 och
med gul färg på koordinaterna `x` och `y`.

```rust
{{#include ../../my-game/examples/move-a-circle.rs:draw}}
```

```admonish info
Macroquad har ett flertal konstanter för vanliga
[färger](https://docs.rs/macroquad/latest/macroquad/color/colors/index.html),
det går också att använda makrot
[`color_u8`](https://docs.rs/macroquad/latest/macroquad/macro.color_u8.html)
för att ange en färg med värden för röd, grön, blå och transparens.
Vilka andra former som går att rita med Macroquad finns beskrivet i
dokumentationen för Macroquads [Shape
API](https://docs.rs/macroquad/latest/macroquad/shapes/index.html).
```

```admonish tip title="Utmaning" class="challenge"
Ändra värdet som adderas till `x` och `y` för att öka eller minska hastigheten
som cirkeln förflyttas.
```

<div class="no-page-break">

## Källkod

Hela källkoden i `main.rs` ska nu se ut så här:

```rust
{{#include ../../my-game/examples/move-a-circle.rs:all}}
```

När du kör programmet så kommer det visas en gul cirkel i mitten av skärmen.
Prova att använda piltangenterna för att flytta omkring bollen.
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/move-a-circle.toml}}

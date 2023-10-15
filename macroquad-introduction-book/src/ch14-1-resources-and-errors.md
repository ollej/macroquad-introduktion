# Resurser och felmmeddelanden

I detta kapitel kommer vi refaktorisera vår kod utan att lägga någon direkt
funktionalitet i spelet. Detta gör vi framförallt för att bygga en grund för
att senare kunna lägga till en laddningsskärm som visar att resurserna håller
på att laddas in. Dessutom kommer vi kunna refaktorisera alla draw-anrop så
att de görs av dom structar som ritas ut. Vi får också fördelen att vi kan
flytta bort kod från vår `main`-funktion som börjar bli svår att överblicka.

## Implementering 

### Resources struct

Till att börja med skapar vi en ny struct som vi kallar `Resources` som kommer
innehålla alla filer vi laddar in från filsystemet. Lägg in den ovanför
`main`-funktionen. Structen har ett fält för varje resurs vi laddar in.

```rust
{{#include ../../mitt-spel/examples/resources-and-errors.rs:struct}}
```

### Resources impl

Direkt under `Resources`-structen skapar vi ett implementationsblock för den.
Till att börja med kommer den bara innehålla en `new`-funktion som laddar in
alla filer och returnerar en instans av structen om allt går bra. Här använder
vi i stort sett samma kod som tidigare låg i `main`-funktionen för att ladda
in alla filer.

Skillnaden är att vi har bytt ut alla `unwrap()` och `expect()`
till `?`-operatorn. Med hjälp av denna kommer felmeddelandet returneras
istället för att avsluta programmet. Det gör att vi kan hantera felmeddelandet
på ett ställe i vår `main`-funktion om vi vill. Felmeddelandet är en enum av
typen `macroquad::Error`.

```admonish info
Vilka felmeddelanden som finns i Macroquad finns beskrivet i [dokumentationen
för macroquad::Error](https://docs.rs/macroquad/latest/macroquad/enum.Error.html).
```

```rust
{{#include ../../mitt-spel/examples/resources-and-errors.rs:impl}}
```

### Returnera fel

För enkelhetens skull kommer vi låta vår `main`-funktion returnera ett
resultat som kan vara ett felmeddelande. Det gör att vi kan använda
`?`-operatorn även i `main`-funktionen. Om `main`-funktionen returnerar ett
felmeddelande kommer applikationen att avslutas och felmeddelandet skrivas ut
på konsollen.

Det vanliga returvärdet i funktionen är `()` som är Rusts "unit typ" som kan
användas om inget värde ska returneras. När funktionen tidigare inte hade
något explicit returvärde så returnerades detta istället implicit.

Om det sista uttrycket i en funktion avslutas med ett semikolon `;` så slängs
dess returvärde bort och `()` returneras istället.

```rust [hl,2]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:main}}
```

```admonish info
Om du undrar hur Rusts unit-typ fungerar så hittar du lite mer information i
[Rusts dokumentation av
unit](https://doc.rust-lang.org/std/primitive.unit.html).
```

### Ta bort `unwrap()`

Vid inladdningen av materialet för shadern använde vi tidigare metoden
`unwrap()` som vi byter ut mot `?`-operatorn för att returnera eventuella fel
istället. Ändringen sker på sista raden i kodexemplet.

```rust [hl,13]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:material}}
```

### Ladda resurser

Nu kommer vi till den intressanta biten i detta avsnitt. Det är dags att byta
ut all kod som laddar in filresurser till att instantiera vår `Resources`
struct istället. Resultat lägger vi variabeln `resources` som vi senare kommer
använda när vi vill komma åt en resurs.

Notera att den använder sig av `await` metoden som kör `new`-metoden som är
`async`. Vi använder oss även här av `?`-operatorn för att direkt returnera om
vi får ett fel.

```rust [hl,2]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:loadresources}}
```

### Uppdatera resursanvändningar

Nu när vi har laddat in resurserna med `Resources` så behöver vi uppdatera
alla ställen som använder en resurs så att de läser från `resources`-variabeln
istället för direkt från en variabel. Vi lägger helt enkelt till `resources.`
framför alla resursnamn.

#### Spelmusik

```rust [hl,2]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:theme}}
```

#### Gränssnittet

Vid definitionen av gränssnittet måste vi uppdatera alla bilder och fonten.
Notera att vi även här har bytt ut `unwrap()` efter `font()`-funktionerna till
att använda `?`-operatorn.

```rust [hl,3,9-10,13,19]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:ui}}
```

#### Laserljud

Laserljudet behöver använda `resources`-structen.

```rust [hl,9]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:sound_laser}}
```

#### Explosioner

För explosionerna måste vi uppdatera referensen till både texturen och
explosionsljudet.

```rust [hl,4,9]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:sound_explosion}}
```

#### Kulor

Uppdatera utritningen av kulorna till att använda texturen från `resources`.

```rust [hl,3]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:bullet_texture}}
```

#### Skeppet

Skeppet behöver också använda texturen från `resources`.

```rust [hl,3]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:ship_texture}}
```

#### Fiender

När fiendera ritas ut behöver `resources` läggas till i referensen av texturen.

```rust [hl,3]
{{#include ../../mitt-spel/examples/resources-and-errors.rs:enemy_small_texture}}
```

Det ska vara allt som behöver ändras den här gången. Vi har nu skapat en
struct som innehåller alla resurser som vi kan använda oss av när vi ritar ut
texturer och spelar upp ljud.

```admonish tip
Istället för att bara avsluta applikationen så kan du pröva att skriva ut
felmeddelandet på skärmen med Macroquads `draw_text` funktion. Tänk på att
programmet då måste fortsätta köra utan att göra något annat än att rita ut
text.
```

## Prova spelet

Spelet ska se ut precis som förut.

<div class="noprint no-page-break">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/resources-and-errors.rs:all}}
```
</details>
</div>


# Fallande fyrkanter

För att det ska hända lite mer i vårt spel är det dags att skapa lite
action. Eftersom hjälten i vårt spel är en modig cirkel så får våra
motståndare bli kantiga fyrkanter som faller ner från toppen av fönstret.

För att hålla reda på vår cirkel och alla fyrkanter så skapar vi en struct som
heter `Shape` som innehåller storlek, hastighet samt x och y-koordinater.

```rust
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
```

Vi kommer använda oss av en slumpgenerator för att avgöra när nya fyrkanter
ska komma in på skärmen. Därför behöver vi seeda slumpgeneratorn så att det
inte blir samma slumptal varje gång. Detta görs i början av `main`-funktionen
med metoden `srand()` som vi skickar in nuvarande tid till som seed. Eftersom
rand-funktionerna inte är med i Macroquads "prelude"-modul måste vi även
importera den längst upp i koden.

```rust
    srand(miniquad::date::now() as u64);
```

I början av `main`-funktionen skapar vi en vektor `squares` som kommer
innehåll alla fyrkanter som ska visas på skärmen. Den ny variabeln `circle`
får representera vår hjälte, den fantastiska cirkeln.

```rust
    let mut squares = vec![];
    let mut circle = Shape {
        size: 30.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
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
        if gen_range(0, 99) >= 95 {
            let size = gen_range::<f32>(15.0, 40.0);
            let square = Shape {
                size,
                speed: gen_range::<f32>(50.0, 150.0),
                x: gen_range::<f32>(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            };
            squares.push(square);
        }
```

Nu kan vi gå igenom hela vektorn med en for-loop och uppdatera y-positionen
med hjälp av fyrkantens hastighet och variabeln `delta_time`. Detta gör att
fyrkanterna kommer åka neråt över skärmen.

```rust
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }
```

Därefter måste vi rensa upp alla fyrkanter som har hamnat utanför skärmen då
det är onödigt att rita ut saker som inte syns. Vi använder oss av metoden
`retain()` på vektorn som tar en funktion som avgör om elementen ska behållas.
Vi kollar att fyrkantens y-värde fortfarande är mindre än höjden på fönstret +
storleken på fyrkanten.

```rust
        squares.retain(|square| square.y < screen_width() + square.size);
```

Till sist lägger vi till en for-loop som går igenom vektorn `squares` och
använder funktionen `draw_rectangle() för att rita ut en rektangel på den
uppdaterade positionen och med rätt storlek. Eftersom rektanglar ritas ut med
x och y från hörnet längst upp till vänster och våra koordinater utgår från
center av fyrkanten så använder vi lite matematik för att räkna ut var dom ska
placeras.

```rust
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
```

## Komplett källkod

Hela programmet ser nu ut så här:

```rust
use macroquad::{prelude::*, rand::*};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 100.0;

    srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut circle = Shape {
        size: 30.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }

        // Clamp X and Y to be within the screen
        circle.x = circle.x.min(screen_width()).max(0.0);
        circle.y = circle.y.min(screen_height()).max(0.0);

        // Generate a new square
        if gen_range(0, 99) >= 95 {
            let size = gen_range::<f32>(15.0, 40.0);
            let square = Shape {
                size,
                speed: gen_range::<f32>(50.0, 150.0),
                x: gen_range::<f32>(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            };
            squares.push(square);
        }

        // Move squares
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // Remove squares below bottom of screen
        squares.retain(|square| square.y < screen_width() + square.size);

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        next_frame().await
    }
}
```



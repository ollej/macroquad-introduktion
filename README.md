# Skapa spel i Rust med Macroquad

Macroquad är ett spelramverk för programmeringsspråket Rust som har allt som
behövs för att skapa ett 2D-spel. De största fördelarna jämfört med andra
spelramverk är att det har väldigt få beroenden och går snabbt att kompilera.
Det stödjer också att göra spel för iOS, Android och webben, förutom desktop
OS som Windows, Mac och Linux. Tack vare att det är så optimerat så går det
även att bygga spel för enklare enheter, som äldre telefoner och små
enkortsdatorer.

## Ditt första Macroquad-program

Nu är det dags att programmera ditt första program med Macroquad. Börja med
att installera programmeringsspråket Rust om du inte redan har gjort det.
Skapa därefter ett nytt Rust-projekt med Cargo och lägg till `macroquad` som
beroende.

```sh
cargo new --bin mitt-spel
cd mitt-spel/
cargo add macroquad
```

Öppna sedan filen `src/main.rs` i din favorit-editor och ändra innehållet till
följande kod:

```rust
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
```

Kör programmet med `cargo run` så ska ett nytt fönster med mörklila bakgrund
öppnas efter att kompileringen är klar.

### Beskrivning av programmet

Första raden används för att importera allt som behövs från Macroquad, vilket
enklast görs med `use macroquad::prelude::*`, men det går också att importera
alla funktioner manuellt.

Attributet `#[macroquad::main("Mitt spel")]` används för att berätta för
Macroquad vilken funktion som ska köras. Macroquad kommer skapa ett fönster
med titeln som anges som argument, och exekvera main-funktionen asynkront.

Inne i main-funktionen körs en evig loop som aldrig avslutas. Inne i loopen
ligger all spellogik som ska köras varje bildruta. I vårt fall rensar vi
bakgrunden till mörklila med funktionen `clear_background(DARKPURPLE)`. I
slutet av loopen används funktionen `next_frame().await` som kommer blocka
exekveringen tills nästa bildruta.

## Far å flyg

Ett spel är inte så roligt utan att det händer något på skärmen. Till att
börja med visar vi en boll som vi kan styra med knapptryckningar.

De första två raderna i main-funktionen använder funktionerna `screen_width()`
och `screen_height()` för att få bredden och höjden på fönstret. Dessa värden
delas med 2 för att få koordinaterna till mitten av skärmen, och tilldelas
till variablerna `x` och `y`. Inne i loopen rensar vi fortfarande skärmen,
vilket måste göras vid varje bildruta. Därefter kommer fyra if-satser som
kollar om piltangerna är nedtryckta och ändrar på variablerna `x` eller `y`
som avgör var cirkeln ska visas. Funktionen `is_key_down()` returnerar sant om
den angivna tangenten är nedtryckt. Dess argument är enumen `KeyCode` som
innehåller alla tangenter som finns på ett tangentbord. Slutligen ritas
cirkeln ut på de angivna koordinaterna med en radie på 15 och med gul färg.

### Källkod

Byt ut innehållet i `main.rs` till följande kod:

```rust
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }

        draw_circle(x, y, 15.0, YELLOW);
        next_frame().await
    }
}
```

När du kör programmet så kommer det visas en gul cirkel i mitten av skärmen.
Prova att använda piltangenterna för att flytta omkring bollen.

## Mjukare rörelser

Eftersom Macroquad kommer rita bildrutor så snabbt som den kan så måste vi
kolla hur lång tid som har gått mellan varje uppdatering för att avgöra hur
långt cirkeln ska förflyttas. Annars kommer vårt spel gå olika fort på olika
datorer, beroende på hur snabbt dom kan köra programmet.

Vi ska därför utöka programmet och lägga till en konstant variabel som avgör
hur snabbt cirkeln ska röra sig. Vi kallar den `MOVEMENT_SPEED` och börjar med
att tilldela den värden `50.0`. Går det för fort eller för sakta kan vi sänka
eller öka detta värde. Därefter använder vi funktionen `get_frame_time()` som
ger oss hur lång tid det har gått sedan föregående bildruta ritades på skärmen
och tilldelar den till variabeln `delta_time`. Förändringen av variablerna `x`
och `y` kan sedan bytas ut till en multiplikation av värdena för
`MOVEMENT_SPEED` och `delta_time` för att få hur långt cirkeln ska förflyttas
under denna bildruta.

Slutligen vill vi också att cirkeln aldrig ska hamna utanför fönstret, därför
begränsar vi variablerna `x` och `y`.

### Källkod

Nu ser vårt program ut så här:

```rust
use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 100.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        x = x.min(screen_width()).max(0.0);
        y = y.min(screen_height()).max(0.0);

        draw_circle(x, y, 15.0, YELLOW);
        next_frame().await
    }
}
```

## Fallande fyrkanter

För att det ska hända lite mer i vårt spel är det dags att skapa lite
action. Eftersom hjälten i vårt spel är en modig cirkel så får våra
motståndare bli kantiga fyrkanter som faller ner från toppen av fönstret.

För att hålla reda på vår cirkel och alla fyrkanter så skapar vi en struct som
heter `Shape` som innehåller storlek, hastighet samt x och y-koordinater.  I
början av `main`-funktionen skapar vi en vektor `squares` som kommer innehåll
alla fyrkanter som ska visas på skärmen. Den ny variabeln `circle` får
representera vår hjälte, den fantastiska cirkeln.

Vi kommer använda oss av en slumpgenerator för att avgöra när nya fyrkanter
ska komma in på skärmen. Därför behöver vi seeda slumpgeneratorn så att det
inte blir samma slumptal varje gång. Detta görs i början av `main`-funktionen
med metoden `srand()` som vi skickar in nuvarande tid till som seed. Eftersom
rand-funktionerna inte är med i Macroquads "prelude"-modul måste vi även
importera den längst upp i koden.

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

Nu kan vi gå igenom hela vektorn med en for-loop och uppdatera y-positionen
med hjälp av fyrkantens hastighet och variabeln `delta_time`. Detta gör att
fyrkanterna kommer åka neråt över skärmen.

Därefter måste vi rensa upp alla fyrkanter som har hamnat utanför skärmen då
det är onödigt att rita ut saker som inte syns. Vi använder oss av metoden
`retain()` på vektorn som tar en funktion som avgör om elementen ska behållas.
Vi kollar att fyrkantens y-värde fortfarande är mindre än höjden på fönstret +
storleken på fyrkanten.

Till sist lägger vi till en for-loop som går igenom vektorn `squares` och
använder funktionen `draw_rectangle() för att rita ut en rektangel på den
uppdaterade positionen och med rätt storlek. Eftersom rektanglar ritas ut med
x och y från hörnet längst upp till vänster och våra koordinater utgår från
center av fyrkanten så använder vi lite matematik för att räkna ut var dom ska
placeras.

### Källkod

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

## Kollission

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

I slutet av huvudloopen lägger vi till vår kontroll av kollissioner. Vi
använder metoden `any()` på vektorn `squares` och kollar om någon fyrkant
kolliderar med vår hjälte cirkeln.

Om det har skett en kollission så gör vi en inre loop som väntar på att
spelaren trycker på mellanslag. Medan vi väntar ritar vi ut texten "Game
Over!" mitt på skärmen.

När spelaren trycker på space-tangenten så tömmer vi vektorn `squares` med
metoden `clear()` och återställer cirkelns x och y-koordinater till mitten av
skärmen. Därefter avbryter vi den inre loopen och återgår till den vanliga
spelloopen.

### Källkoden

```rust
use macroquad::{prelude::*, rand::*};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }
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

        // Check for collissions
        if squares.iter().any(|square| circle.collides_with(square)) {
            let text = "Game Over!";
            let text_dimensions = measure_text(text, None, 60, 1.0);
            loop {
                clear_background(DARKPURPLE);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
                if is_key_down(KeyCode::Space) {
                    squares.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    break;
                }
                next_frame().await
            }
        }

        next_frame().await
    }
}
```



## Skjuta


## Inertia movement


## Poäng


## Startmeny


## Byt till texturer


## Lägg till ljud

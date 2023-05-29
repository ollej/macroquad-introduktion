# Grafisk meny

Macroquad har ett inbyggt system för att rita upp ett grafiskt gränssnitt som
där utseendet enkelt kan ändras med hjälp av bilder. Vi ska använda det för
att skapa en grafisk huvudmeny för vårt spel. Det kommer vara ganska mycket
kod för att definiera hur gränssnittet ska se ut. Att använda det kräver dock
inte riktigt lika mycket kod.

Menyn kommer bestå av ett fönster centrerat på skärmen, med texten "Huvudmeny"
i titeln, och kommer innehålla två knappar, en för att "Spela" och en för att
"Avsluta". Utseendet kommer beskrivas med kod, och använder bilder för att
skapa utseendet. Gränssnitt byggs upp med hjälp av olika widgets som label,
button, editbox och combobox.

## Implementering 

Till att börja med måste vi importera det vi behöver från `ui`-modulen.

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:import}}
```

### Ladda in resurser

Efter att ljuden har laddats in ska vi ladda in fonten och de bilder som
behövs för att rita upp gränssnittet. Vi har en bild för att skapa ett
fönster, `window_background.png`, en bild för att rita upp knappar,
`button_background.png` och till sist en bild som används när en knapp är
nedtryckt, `button_clicked_background.png`. Bilder laddas in med funktionen
`load_image()` och binärfiler med `load_file()`. Både bilder och filer laddas
in asynkront och kan returnera fel, därför använder vi oss av `await` och
`unwrap()`. Lyckas vi inte ladda in det som behövs för att rita upp huvudmenyn
kan vi avsluta programmet direkt.

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:loadresources}}
```

### Skapa ett Skin

Innan loopen måste vi definiera hur vårt gränssnitt ska se ut. Vi bygger upp
`Style`-structar för fönstret, knappar och texter. Därefter skapar vi ett
`Skin` med stilarna.

Vi använder oss av funktionen `root_ui()` som kommer rita widgets sist i varje
frame med en "default" kamera och koordinatsystemet
`(0..screen_width(), 0..screen_height())`.

#### Utseende på fönster

För att bygga en stil använder man en `StyleBuilder` som har hjälpmetoder för
att definiera alla delar av stilen. Vi får tillgång till den genom att
använda metoden `style_builder()` på `root_ui()`. De värden som inte sätts
kommer att använda samma värden som default-utseendet.

Vi använder metoden `background()` för att sätta bilden som ska användas för
att rita ut fönstret. Sen måste vi använda `background_margin()` för att
definiera vilka delar av bilden som inte ska stretchas ut när fönstret ändrar
storlek. Det använder vi för att kanterna på fönstret ska se bra ut.

Med metoden `margin()` sätts marginaler för innehållet. Dessa värden kan vara
negativa för att rita ut innehåll på fönstrets bårder.

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:windowstyle}}
```

```admonish info
Det finns många fler metoder för att definiera stilar, som finns beskrivna i
dokumentationen för [Macroquads
`StyleBuilder`](https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.StyleBuilder.html)
```

#### Utseende på knappar

I definitionen för knappar använder vi två bilder, med `background()` sätter
vi grundbilden och med `background_clicked()` sätter vi bilden som ska
användas när knappen är nedtryckt.

Vi behöver `background_margin()` och `margin()` för att kunna stretcha ut
bilden över hela textinnehållet. Utseendet på texten sätter vi med `font()`,
`text_color()` och `font_size()`.

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:buttonstyle}}
```

#### Utseende på text

Vanlig text som ska presenteras i gränssnittet använder `label_style`. Vi
använder samma font som för knappar, men med en lite mindre storlek.

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:labelstyle}}
```

#### Definiera ett Skin

Nu kan vi skapa ett `Skin` med hjälp av `window_style`, `button_style` och
`label_style`. Övriga stilar i vårt skin låter vi vara som dom är då vi inte
kommer använda dom just nu.

Vi sätter vårt skin som aktuellt skin med `push_skin()`. Vi kommer bara
använda oss av en stil, men för att byta mellan olika stilar mellan fönster
kan man definiera flera skins och använda `push_skin()` och `pop_skin()` för
att byta mellan dem.

Vi sätter också variabeln `window_size` som kommer användas för sätta
fönstrets storlek.

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:uiskin}}
```

```admonish info
Det går att ändra utseendet på fler delar av gränssnittet, som textrutor,
dropboxar med mera. Mer information finns i [dokumentationen av structen
Skin](https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.Skin.html).
```

### Bygg upp menyn

Nu kan vi skapa en meny genom att rita ut ett fönster med två knappar och en
rubrik. Innehållet i matchningen av `GameState::MainMenu` kan bytas ut mot
nedanstående kod.

Först skapar vi ett fönster med anropet `root_ui().window()`. Den funktionen
tar ett id som genereras med macrot `hash!`, en position som vi räknar ut
baserat på fönsterstorleken och skärmens dimensioner och en `Vec2` som
beskriver fönstrets storlek. Till sist tar den en funktion som används för att
rita upp fönstret.

#### Fönstertitel

Inne i funktionen skapar vi först en titel på fönstret med widgeten `Label`
som vi kan skapa med metoden `ul.label()`. Metoden tar två argument, först en
`Vec2` med positionen för var den ska placeras, och texten som ska visas. Det
går att skicka in `None` som position, då kommer den få en placering relativ
till tidigare widgets. Här använder vi en negativ Y-position för att den ska
hamna i fönstrets titelrad.

```admonish info
Widgets går också att skapa genom att instantiera ett objekt och använda
builder-metoder.

`widgets::Button::new("Spela").position(vec2(45.0, 25.0)).ui(ui);`
```

#### Knappar

Sen ritar vi ut en knapp för att börja spela. Metoden `ui.button()` returnerar
`true` om knappen är nedtryckt. Det använder vi oss för att sätta ett nytt
`GameState` och starta ett nytt spel.

Till sist skapar vi knappen "Avsluta" som avslutar programmet om spelaren
klickar på den.

```rust [hl,2-11,19-20,22-24]
{{#include ../../mitt-spel/examples/menu-ui.rs:menu}}
```

```admonish info
Det finns en mängd olika widgets som kan användas för att skapa gränssnitt.
Läs mer om vad som finns tillgängligt i [dokumentationen av structen
`Ui`](https://docs.rs/macroquad/0.3.25/macroquad/ui/struct.Ui.html).
```

## Prova spelet

När spelet startar nu så finns det en grafisk huvudmeny där spelaren kan välja
att starta ett spel eller avsluta programmet.

<div class="noprint no-page-break">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/menu-ui.rs:all}}
```
</details>
</div>


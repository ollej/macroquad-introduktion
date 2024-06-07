# Lägg ut ditt spel på webben

Eftersom det går att kompilera Macroquad-spel till WebAssembly så är det
möjligt att köra spelet i en webbläsare. Här kommer instruktioner för hur du
kan skapa en webbsida som kör ditt spel. Denna sida kan du sedan lägga upp på
ditt webhotell så att andra enkelt kan spela ditt spel utan att behöva ladda
ner något.

## Installera WASM build target

Till att börja med måste du installera en build target för WASM. Det görs
med kommandot `rustup`.

```sh
rustup target add wasm32-unknown-unknown
```

## Bygg en WASM-binär

Nu kan du använda WASM build targeten för att bygga en WASM-binärfil som sedan
kan laddas från en webbsida.

```sh
cargo build --release --target wasm32-unknown-unknown
```

Binärfilen som skapas kommer att placeras i katalogen
`target/wasm32-unknown-unknown/release/` med filändelsen `.wasm`.

## Kopiera WASM-binären

Du måste kopiera binärfilen som skapades i steget ovan till roten av din
crate, samma katalog som `assets`-katalogen ligger i.

Om du har döpt din crate till något annat än "my-game" så kommer namnet på
WASM-filen vara samma som namnet på din crate, med filändelsen `.wasm`.

```sh
cp target/wasm32-unknown-unknown/release/my-game.wasm .
```

## Skapa en webbsida

Det behövs en HTML-sida för att ladda in WASM-binären. Den behöver ladda in en
javascript-fil från Macroquad som innehåller kod för att WASM-binären ska
kunna kommunicera med webbläsaren. Det behövs även en `canvas`-tagg som
Macroquad använder för att rita ut grafiken. Kom ihåg att byta ut namnet på
binärfilen i `load`-anropet från `my-game.wasm` till det du döpt ditt spel
till. Det kommer vara samma som namnet på din crate.

Skapa en fil med namnet `index.html` i roten på din crate med följande
innehåll:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>My Game</title>
    <style>
        html,
        body,
        canvas {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>
<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/macroquad/blob/master/js/mq_js_bundle.js -->
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script>load("my-game.wasm");</script> <!-- Your compiled WASM binary -->
</body>
</html>
```

## Testkör spelet i din webbläsare

Nu kan du starta en webbserver och ladda sidan i din webbläsare.

### Installera en enkel webbserver

För att serva ditt spel lokalt på din dator kan du installera en enkel
webbserver med nedanstående kommando. Detta är enbart för att kunna testköra
spelet innan du lägger ut det på ett riktigt webbhotell.

```sh
cargo install basic-http-server
```

### Kör igång webbservern

Detta kommando skriver ut vilken adress du kommer komma åt webbsidan på. Öppna
din webbläsare och surfa till den adressen, till exempel
`http://localhost:4000`. Spelet kommer nu köras i din webbläsare istället för
som en egen applikation.

```sh
basic-http-server .
```

### Lägg ut ditt spel

Om du har tillgång till ett webbhotell kan du lägga ut filerna där så att andra
kan spela ditt spel. Du behöver lägga upp html-filen, WASM-filen och katalogen
`assets`.

```
index.html
my-game.wasm
assets/*
```

```admonish note
Vi påminner om att det redan i [kapitel 1](ch1-first-program.md#publicera-på-webben-om-du-vill) finns instruktioner för hur du publicerar spelet till webben utan webbhotell. Se i så fall till att du använder den uppdaterade `deploy.yml` från [kapitel 10 – Grafik](ch11-graphics.md#uppdatera-webbpubliceringen).
```

# Bygg för Android-mobiler

Med Macroquad går det även att bygga för att spela spelet på Android-mobiler.
Vi kommer bygga en APK-fil som kan installeras på Android-telefoner eller
läggas ut på Google Play Store. Vi kommer bara beskriva hur man bygger med
hjälp av Docker, så det kommer krävas att det är installerat.

Tänk på att en mobil plattform inte har ett fysiskt tangentbord och därför
behövs det byggas in stöd för att styra spelet med touch-kontroller.

```admonish note
Läs om funktionen `touches()` i [Macroquads
dokumentation](https://docs.rs/macroquad/latest/macroquad/input/index.html)
för information om hur touch-kontroller fungerar.
```

## Installera docker image

Innan du kan börja bygga en APK-fil för Android behöver du hämta hem 
Docker-imagen `notfl3/cargo-apk`.

```sh
docker pull notfl3/cargo-apk
```

## Bygg APK-fil

Med detta kommando kan du bygga en APK-fil. Det kommer ta en stund då den
gör tre fulla byggen, en för varje Android target.

```sh
docker run 
  --rm 
  -v $(pwd):/root/src 
  -w /root/src 
  notfl3/cargo-apk cargo quad-apk build --release
```

Detta kommer skapa en APK-fil i katalogen
`target/android-artifacts/release/apk`.

## Konfiguration

För att Android ska hitta alla assets måste en konfiguration läggas till i
`Cargo.toml` som beskriver var assets-katalogen ligger.

```toml
[package.metadata.android]
assets = "assets/"
```

```admonish info
På Macroquads hemsida finns en ingående beskrivning om hur man [bygger för
Android](https://macroquad.rs/articles/android/). Där finns tips för att
snabba upp bygget, hur man bygger manuellt utan Docker och hur man signerar
APK-filen för att kunna lägga upp den på Google Play Store.
```

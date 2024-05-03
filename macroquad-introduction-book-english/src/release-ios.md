# Bygg för iOS

Det går även att bygga ditt Macroquad-spel för att köra på iPhone-mobiler och
iPads. 

```admonish info
Utförligare information om att bygga för iOS finns i artikeln [Macroquad on
iOS](https://macroquad.rs/articles/ios/) på Macroquads hemsida. Där finns
information om hur man får tillgång till loggar, bygger för riktiga enheter
och signerar appen.
```

## Skapa en katalog

En iOS-applikation är en vanlig katalog/mapp med filändelse `.app`.

```sh
mkdir MittSpel.app
```

För vårt spel är filstrukturen i `.app`-mappen samma som när vi kör
spelet med `cargo run` från roten av craten. Det vill säga, binärfilen och
`assets`-mappen skall placeras bredvid varandra. Det behövs även en
`Info.plist`-fil.

Börja med att lägga `assets` på plats:

```sh
cp -r assets MittSpel.app 
```

## Bygg binärfilen

Du behöver Rusts targets för iOS. För simulatorn används Intel-binärer och
för en fysisk enhet används ARM-binärer. Vi går enbart igenom hur du testar
på simulatorn i den här guiden. Att få igång testning på en fysisk enhet är
ett kapitel för sig. Se [Macroquad on
iOS](https://macroquad.rs/articles/ios/) för information om det.

```sh
rustup target add x86_64-apple-ios
```

Nu kan du bygga en exekverbar binärfil för iOS Simulator med följande
kommando.

```sh
cargo build --release --target x86_64-apple-ios
```

## Kopiera binärfilen

Kopiera in den exekverbara binärfil som byggdes ovan till spelets mapp.

```sh
cp target/x86_64-apple-ios/release/mitt-spel MittSpel.app
```

## Skapa Info.plist

Skapa en textfil för appens metadata med namnet `Info.plist` i
`MittSpel.app`-mappen med följande innehåll:

```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>CFBundleExecutable</key>
<string>mitt-spel</string>
<key>CFBundleIdentifier</key>
<string>com.mittspel</string>
<key>CFBundleName</key>
<string>mittspel</string>
<key>CFBundleVersion</key>
<string>1</string>
<key>CFBundleShortVersionString</key>
<string>1.0</string>
</dict>
</plist>
```

## Sätt upp simulatorn

För det här steget behöver du ha **XCode** och minst en simulator-image
installerad. XCode hittar du i **App Store**-appen. Du kan lägga till
simulatorer via kommandoraden, eller via XCode. I version 15.1 av XCode
kan du göra det via **Settings...** -> **Platforms** och sedan välja någon
av iOS-versionerna där. Där finns en även knapp (`+`) för att lägga till
ytterligare iOS-versioner.

För att sätta upp via kommandoraden så kör först `xcrun simctl list` för att
se en lista på alla tillgängliga simulatorer. Kopiera den långa hex-koden för
den simulator du vill boota och använd det som argument till `xcrun simctl
boot`. Detta behöver bara göras första gången du ska köra simulatorn.

```bash
xcrun simctl list
xcrun simctl boot <hex string>
```

## Kör igång simulatorn

Kommandot vi kommer att använda för att installera och köra spelet, `xcrun
simctl`, väljer ut simulator med argumentet `booted`. Det innebär att du
först behöver starta en simulator, och för att göra det förutsägbart bör du
ha endast en simulator igång. Även detta går att göra via kommandoraden,
men det enklaste är nog att starta **Simulator**-appen och sedan starta en
simulator via **File** -> **Open Simulator**.

För att starta simulatorn via commandline använd följande kommando:

```bash
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app/
```

## Installera spelet

Du kan installera spelet genom att dra mappen `MittSpel.app` och släppa den
i den startade simulatorn. Men eftersom du antagligen kommer vilja
installera om ofta är det effektivare att använda kommandoraden för detta:

```sh
xcrun simctl install booted MittSpel.app/
```

## Starta spelet

Även detta kan du göra i den startade simulatorn eller via kommandoraden.
I vår `Info.plist` specificerade vi `CFBundleIdentifier` som
`com.mittspel`.

```sh
xcrun simctl launch booted com.mittspel
```

```admonish note
Du kommer märka att spelet inte är anpassat för att köras på en mobil
plattform än. Du kan förslagsvis börja med att läsa om funktionen `touches()`
i
[Macroquads
dokumentation](https://docs.rs/macroquad/latest/macroquad/input/index.html)
för information om hur touch-kontroller fungerar.
```


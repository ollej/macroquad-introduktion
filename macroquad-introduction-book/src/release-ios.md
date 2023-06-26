# Bygg för iOS

Det går även att bygga ditt Macroquad-spel för att köra på iPhone-mobiler och
iPads. 

## Skapa en katalog

En iOS-applikation är en vanlig katalog som innehåller alla filer som behövs
för spelet.

```sh
mkdir MittSpel.app
```

## Bygg binärfilen

Om du inte redan har installerat Rusts target för iOS så börja med att göra
det.

```sh
rustup target add x86_64-apple-ios
```

Nu kan du bygga en exekverbar binärfil för iOS med följande kommando.

```sh
cargo build --release --target x86_64-apple-ios
```

## Kopiera binärfilen

Kopiera in den exekverbara binärfil som byggdes ovan till spelets katalog.

```sh
cp target/x86_64-apple-ios/release/mitt-spel MittSpel.app
```

## Skapa Info.plist

Skapa en textfil för appens metadata med namnet `Info.plist` i
`MittSpel.app`-katalogen med följande innehåll:

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

Kör följande kod för att sätta upp simulatorn, detta behöver bara göras en
gång. Byt ut `LONG_HEX_ID` mot det UUID för den enhet du vill simulera från
list-kommandot.

```sh
xcrun simctl list
xcrun simctl boot LONG_HEX_ID
```

## Öppna simulatorn

Kör följande kommando för att köra igång iOS-simulatorn för att köra din app.

```sh
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app/
```

## Kör appen

Använd nedanstående kommandon efter varje bygge för att köra din app.

```sh
xcrun simctl install booted MittSpel.app/
xcrun simctl launch booted com.mittspel
```

```admonish info
Utförligare information om att bygga för iOS finns i artikeln [Macroquad on
iOS](https://macroquad.rs/articles/ios/) på Macroquads hemsida. Där finns
information om hur man får tillgång till loggar, bygger för riktiga enheter
och signerar appen.
```

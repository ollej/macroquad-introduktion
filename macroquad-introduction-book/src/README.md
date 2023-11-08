# Koda ett spel i Rust med Macroquad

![Ferris the rustacean with a santa hat holding a game controller](images/ferris-gamer-santa.png#center)

## Välkommen till Agicals julkalender 2023!

Häng med på Agicals spelmakarpyssel och bygg ett eget shoot 'em up-spel i
Rust. Varje dag under december kommer vi lägga ut ett nytt kapitel och lagom
till julafton kommer vi skrivit ett färdigt spel.

Guiden är skriven av [Olle Wreede](mailto:olle.wreede@agical.se) på
[Agical](https://www.agical.se/). 

## Spelramverket Macroquad

Macroquad är ett spelramverk för programmeringsspråket Rust som har allt som
behövs för att skapa ett 2D-spel. De största fördelarna jämfört med andra
spelramverk är att det har väldigt få beroenden och går snabbt att kompilera.
Det stödjer också att göra spel för iOS, Android och webben, förutom desktop
OS som Windows, Mac och Linux. Det behövs ingen plattformsspecifik kod för att
det ska fungera, all kod är alltid samma. Tack vare att det är så optimerat så
går det även att bygga spel för enklare enheter, som äldre telefoner och små
enkortsdatorer. Det ingår även ett UI-ramverk för att göra grafiska UI där
utseendet enkelt kan ändras.

Denna guide förutsätter en viss förkunskap i Rust. Det går att läsa mer om
Rust i [Rust-boken](https://doc.rust-lang.org/book/) som finns att läsa
online. Jag kan även rekommendera boken
[Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/)
av Herbert Wolverson där man får lära sig Rust genom att skriva ett spel.

```admonish info
På [Macroquads hemsida](https://macroquad.rs) finns
[exempel](https://macroquad.rs/examples/) som visar hur Macroquad fungerar och
[dokumentation](https://docs.rs/macroquad/latest/macroquad/) av dess API.
```

## Spelmakarguide

I denna guide kommer vi bygga ett spel från grunden genom att lägga till
lite mer funktionalitet i varje kapitel. Till att börja med kommer det vara
väldigt rudimentärt, men i slutet av guiden kommer det vara ett komplett spel
med grafik, ljud och allt som hör till.

Spelet vi kommer skapa är ett klassiskt shoot 'em up där spelaren ska flyga
ett rymdskepp och skjuta ner fiender.

```admonish tip title="Utmaning" class="challenge"
Detta är läraren Ferris som kommer dyka upp i slutet av varje kapitel för att
ge dig en liten extra utmaning. Det är valfritt att utföra utmaningen, det
kommer inte behövas för att kunna fortsätta till nästa kapitel.
```

```admonish note title="Notera"
Denna guide är skriven för version 0.4 av Macroquad. Eftersom Macroquad är
under aktiv utveckling kommer den inte gälla för v0.5 och senare.
```

<a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/"><img alt="Creative Commons-licens" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-sa/4.0/88x31.png" /></a><br />Detta verk är licensierat under en <a rel="license" href="http://creativecommons.org/licenses/by-nc-sa/4.0/">Creative Commons Erkännande-IckeKommersiell-DelaLika 4.0 Internationell Licens</a>.

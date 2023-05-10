# Koda ett spel i Rust med Macroquad

Macroquad är ett spelramverk för programmeringsspråket Rust som har allt som
behövs för att skapa ett 2D-spel. De största fördelarna jämfört med andra
spelramverk är att det har väldigt få beroenden och går snabbt att kompilera.
Det stödjer också att göra spel för iOS, Android och webben, förutom desktop
OS som Windows, Mac och Linux. Tack vare att det är så optimerat så går det
även att bygga spel för enklare enheter, som äldre telefoner och små
enkortsdatorer.

Denna guide förutsätter en viss förkunskap i Rust. Det går att läsa mer om
Rust i [Rust-boken](https://doc.rust-lang.org/book/). Jag kan även
rekommendera boken [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/)
av Herbert Wolverson där man får lära sig Rust genom att skriva ett spel.

I denna guide kommer vi bygga ett spel från grunden genom att lägga till
lite mer funktionalitet i varje kapitel. Till att börja med kommer det vara
väldigt rudimentärt, men i slutet av guiden kommer det vara ett komplett spel
med grafik, ljud och allt som hör till.

```admonish info
På [Macroquads hemsida](https://macroquad.rs) finns
[exempel](https://macroquad.rs/examples/) som visar hur Macroquad fungerar och
[dokumentation](https://docs.rs/macroquad/latest/macroquad/) av dess API.
```

# Koda ett spel i Rust med Macroquad

<div class="noprint">

![Boksläpp 26:e februari kl 18 på Agicals kontor, OSA till bok@agical.se](images/bokrelease.png)

Den [26:e februari kl 18](https://events.agical.se/events/bokslapp-game-development-in-rust-with-macroquad-20250226/)
kommer det vara boksläpp för den tryckta versionen av denna guide på engelska.
Tillställningen kommer hållas på Agicals kontor i Gamla stan i Stockholm. OSA
till [bok@agical.se](mailto:bok@agical.se) om du vill vara med på boksläppet
och skriv även om du vill köpa ett exemplar av boken.

Efter boksläppet kommer boken finnas tillgänglig att beställa online.

</div>

![Ferris the rustacean holding a game controller](images/ferris-gamer.png#center)

Lär dig koda ett eget shoot 'em up-spel i programmeringsspråket Rust. I denna
guide kommer du skriva ett komplett spel från grunden och samtidigt lära dig
hur spelramverket Macroquad fungerar. När du gått igenom hela guiden kommer du
ha ett spelbart spel som går att spela på stationära datorer, mobiler och
webben.

Guiden är skriven av [Olle Wreede](mailto:olle.wreede@agical.se) på
[Agical](https://www.agical.se/). 

```admonish info
Denna guide finns även att läsa på [engelska](https://mq.agical.se/).
```

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

<p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><a property="dct:title" rel="cc:attributionURL" href="https://macroquad-introduktion.agical.se/">Koda ett spel i Rust med Macroquad</a> by <a rel="cc:attributionURL dct:creator" property="cc:attributionName" href="https://olle.wreede.se">Olle Wreede</a> licensieras med <a href="https://creativecommons.org/licenses/by-sa/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">Creative Commons Attribution-ShareAlike 4.0 International<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/sa.svg?ref=chooser-v1" alt=""></a></p> 

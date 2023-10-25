# Game state

![Screenshot](images/game-state.gif#center)

Innan vi lägger till någon ny funktionalitet i vårt spel så är det dags för
lite refaktorisering. För att det ska bli enklare att hantera spelets
tillstånd så inför vi en enum vid namn `GameState` som håller reda på om
spelet pågår eller om det har blivit game over. Tack vare detta kan vi ta bort
vår `gameover` variabel, och lägga till tillstånd för en meny och pausa
spelet.

## Implementering

### Enum för game state

Börja med att lägga till en enum kallad `GameState` under implementationen av
`Shape`. Den innehåller alla fyra tillstånd som spelet kan vara i.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:stateenum}}
```

### Variabel för GameState
Ersätt raden som deklarerar variabeln `gameover` med en deklarering av en ny
`game_state` variabel. Till att börja med sätter vi den till tillståndet
`GameState::MainMenu` så att vi väntar på att spelaren trycker mellanslag
innan spelet börjar.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:statevariable}}
```

### Matcha på GameState

Koden inne i spelloopen ska nu ersättas med en matchning på variabeln
`game_state`. Den måste hantera alla tillstånd i enumen. Senare ska vi införa
koden från tidigare steg inne i de olika blocken. Behåll anropet till rensning
av skärmen i början av loopen, och anropet till `next_frame().await` i slutet.

```rust [hl,3-16]
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                ...
            }
            GameState::Playing => {
                ...
            }
            GameState::Paused => {
                ...
            }
            GameState::GameOver => {
                ...
            }
        }

        next_frame().await
```

### Huvudmeny

Nu ska vi lägga till kod i varje block i matchningen för att hantera varje
tillstånd. När spelet börjar kommer spelet vara i tillståndet
`GameState::MainMenu`. Vi börjar med att kolla om `Escape` är nedtryckt så kan
vi avsluta spelet. Om spelaren trycker på mellanslagstangenten tilldelar vi
det nya tillståndet `GameState::Playing` till variabeln `game_state`. Vi
passar även på att nollställa alla spelvariabler. Till sist skriver ut texten
"Tryck mellanslag" i mitten av skärmen.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:mainmenu}}
```

### Pågående spel

Nu ska vi lägga tillbaka koden för spelet, det är samma som större delen av
spelloopen från förra kapitlet. Dock ska inte koden som hanterar game over vara
med då vi kommer lägga in det nedan i tillståndet för `GameState::Playing`. Vi
lägger också till en kontroll om spelaren tryckt på `Escape` och byter
tillstånd till `GameState::Paused`.

```rust [hl,1,24-26,108]
{{#include ../../mitt-spel/examples/game-state.rs:playing}}
```

### Pausa spelet

Många spel har en möjlighet att pausa, så vi passar på att lägga in stöd för
det även i vårat spel. I pausat läge kollar vi om spelaren trycker på
`Escape`, om så är fallet så sätter vi tillståndet till `GameState::Playing`
så att spelet kan fortsätta igen. Sen skriver vi ut en text på skärmen om att
spelet är pausat.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:paused}}
```

### Game Over

Till sist ska vi hantera vad som händer när det blir game over. Om spelaren
trycker på mellanslag så byter vi tillstånd till `GameState::MainMenu` så att
spelaren kan börja ett nytt spel eller avsluta spelet. Sen skriver vi ut
texten på skärmen som tidigare.

```rust
{{#include ../../mitt-spel/examples/game-state.rs:gameover}}
```

```admonish note
Eftersom tillstånden för `Playing` och `GameOver` är separerade nu så visas
inte någonting från spelet när det är game over.
```

```admonish tip title="Utmaning" class="challenge"
Nu när det finns en startmeny så kan du hitta på ett namn på ditt spel och
skriva ut det med stor text på övre delen av skärmen i tillståndet för
`GameState::MainMenu`.
```

<div class="noprint">

## Kompletta källkoden

<details>
  <summary>Klicka för att visa hela källkoden</summary>

```rust
{{#include ../../mitt-spel/examples/game-state.rs:all}}
```
</details>
</div>

## Quiz

Testa dina nya kunskaper genom att svara på följande quiz innan du går vidare.

{{#quiz ../quizzes/game-state.toml}}

# Resurser

![Svart bild med texten "Laddar resurser" följt av tre blinkande punkter](images/coroutines-and-storage.gif#center)

Då det börjar bli väldigt mycket kod i vår `main`-funktion börjar det bli dags
att strukturera om koden lite.

Vi kommer börja med att flytta inladdningen av filresurser till en egen
struct. samtidigt passar vi på att byta ut alla `unwrap()` och `expect()` till
att använda `?`-operatorn för hantering av felmmeddelanden.

Därefter kommer vi använda oss av en `coroutine` för att ladda resurserna i
bakgrunden samtidigt som vi visar en laddningssnurra på skärmen.

Avslutningsvis kommer vi att använda en `storage` för att resurserna ska
bli tillgängliga i hela koden utan att vi behöver skicka runt den till alla
ställen den behövs. Det gör att vi senare kan refaktorisera vår kod till att
låta alla sprites rita ut sig själva.


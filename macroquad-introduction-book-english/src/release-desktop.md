# Bygg ditt spel för desktopdatorer

Macroquad stödjer flera olika desktopplattformar, som Windows, Linux och
MacOS. Det går att korskompilera för andra plattformar än den du själv kör på,
men det kan kräva en del andra verktyg och tas inte upp i den här guiden.
Enklast kan vara att använda ett byggsystem som har stöd för flera olika
plattformar.

## Bygg på Windows

Om du ska bygga ditt spel för att kunna köras på Windows så måste en Rust
build target installeras. Både MSVC och GNU build targets stöds.

### Bygg med Windows GNU target

Innan första bygget måste rätt target installeras, detta behöver bara göras en
gång.

```sh
rustup target add x86_64-pc-windows-gnu
```

För att bygga spelet kan du köra detta kommando:

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

Binärfilen som skapas kommer att placeras i katalogen
`target/x86_64-pc-windows-gnu/release/`.

### Bygg med Windows MSVC target

Innan första bygget måste rätt target installeras, detta behöver bara göras en
gång.

```sh
rustup target add x86_64-pc-windows-msvc
``` 

För att bygga spelet kan du köra detta kommando:

```sh
cargo build --release --target x86_64-pc-windows-msvc
```

Binärfilen som skapas kommer att placeras i katalogen
`target/x86_64-pc-windows-msvc/release/`.

## Bygg på Linux

För att kunna bygga ett spel med Macroquad på Linux så krävs det att några
utvecklingspaket är installerade. Nedan visas hur dessa paket kan installeras
med några vanliga Linux-distributioner.

### Installera paket

#### Ubuntu

Dessa systempaket måste installeras för att bygga på Ubuntu.

```sh
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

#### Fedora

Dessa systempaket måste installeras för att bygga på Fedora.

```sh
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel
```

#### Arch Linux

Dessa systempaket måste installeras för att bygga på Arch Linux.

```sh
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

### Bygg med Linux GNU target

Innan första bygget måste rätt target installeras, detta behöver bara göras en
gång.

```sh
rustup target add x86_64-unknown-linux-gnu
```

För att bygga spelet kan du köra detta kommando:

```sh
cargo build --release --target x86_64-unknown-linux-gnu
```

Binärfilen som skapas kommer att placeras i katalogen
`target/x86_64-unknown-linux-gnu/release/`.

## Bygg på MacOS

För MacOS finns det två möjliga targets, `x86_64-apple-darwin` som används för
äldre Intel-baserade Mac-datorer, och `aarch64-apple-darwin` som bygger för att
köras på nyare Apple Silicon-baserade Mac-datoer.

### Bygg med x86-64 Apple Darwin target

Innan första bygget måste rätt target installeras, detta behöver bara göras en
gång.

```sh
rustup target add x86_64-apple-darwin
```

För att bygga spelet kan du köra detta kommando:

```sh
cargo build --release --target x86_64-apple-darwin
```

Binärfilen som skapas kommer att placeras i katalogen
`target/x86_64-apple-darwin/release/`.

### Bygg med aarch64 Apple Darwin target

Innan första bygget måste rätt target installeras, detta behöver bara göras en
gång.

```sh
rustup target add aarch64-apple-darwin
```

För att bygga spelet kan du köra detta kommando:

```sh
cargo build --release --target aarch64-apple-darwin
```

Binärfilen som skapas kommer att placeras i katalogen
`target/aarch64-apple-darwin/release/`.

## Paketera spelet

För att dela med dig av ditt spel till andra behöver du paketera binärfilen
tillsammans med alla assets som behövs för att köra spelet. Här nedan visas
några exempel på hur det kan göras från ett terminalfönster.

### Windows

```sh
cp target/x86_64-pc-windows-gnu/release/my-game.exe ./
tar -c -a -f my-game-win.zip my-game.exe assets/*
```

### Linux

```sh
cp target/x86_64-pc-linux-gnu/release/my-game ./
tar -zcf my-game-linux.zip my-game assets/*
```

### Mac

```sh
cp target/aarch64-apple-darwin/release/my-game ./
zip -r my-game-mac.zip my-game assets/*
```


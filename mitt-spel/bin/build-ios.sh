#!/usr/bin/env sh

set -euo pipefail

# Setup game variables
GAMEAPP=MittSpel.app
GAMENAME=com.mittspel
DEVICE=D5E5C93F-02D8-462C-9CF4-304DC38BB1D1

VERBOSE=""

function show_help {
    echo "USAGE: $0 [-B] [-b] [-c] [-i] [-l] [-h] [-n <name>] [-a <app>]"
    echo "  -B  Boot device $DEVICE"
    echo "  -n <name>   Use <name> as appname to boot"
    echo "  -a <app>   Use <app> as app to install"
    echo "  -d <device>   Use <device> to boot"
    echo "  -b  Build game with cargo"
    echo "  -c  Copy binary to game dir"
    echo "  -i  Install game in simulator"
    echo "  -l  Launch game in simulator"
    echo "  -v  Verbose"
    echo "  -h  Show this help"
}

function verbose_print {
    if [ ! -z "$VERBOSE" ]; then
        echo "$1"
    fi
}

# Boots the configured device simulator
function boot_device {
    verbose_print "Booting device $DEVICE ..."
    xcrun simctl boot $DEVICE
}

# Build the binary with Cargo
function build_binary {
    verbose_print "Building iOS binary..."
    cargo build --quiet --release --target x86_64-apple-ios
}

# Copy binary to game directory
function copy_binary {
    verbose_print "Copying iOS binary..."
    cp target/x86_64-apple-ios/release/mitt-spel "$GAMEAPP"
}

# Install game in simulator
function install_game {
    verbose_print "Install app $GAMEAPP in simulator..."
    xcrun simctl install booted "$GAMEAPP/"
}

# Launch game in simulator
function launch_game {
    verbose_print "Launch app $GAMEAPP in simulator..."
    xcrun simctl launch booted $GAMENAME
}

# Read command line flags
while getopts n:a:vBbcilh flag
do
    case "${flag}" in
        v) VERBOSE=1;;
        a) GAMEAPP="${OPTARG}";;
        n) GAMENAME="${OPTARG}";;
        B) boot_device;;
        b) build_binary;;
        c) copy_binary;;
        i) install_game;;
        l) launch_game;;
        h) show_help;;
    esac
done

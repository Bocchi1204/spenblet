#!/bin/sh
set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
release_url=https://github.com/Bocchi1204/spenblet/releases/download/v1.0.0-beta.1/spenblet-1.0.0-beta.1.apk

if command -v pacman >/dev/null 2>&1; then
    sudo pacman -S --needed android-tools android-udev rust curl
elif command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y adb cargo curl
else
    printf '%s\n' 'Unsupported package manager. Install adb, curl, and Rust manually.' >&2
    exit 1
fi

cargo build --manifest-path "$project_dir/linux/Cargo.toml" --release
sudo modprobe uinput
sudo "$project_dir/scripts/install-udev.sh"
sudo install -Dm755 "$project_dir/linux/target/release/spenblet-daemon" /usr/local/bin/spenblet-daemon
sudo install -Dm755 "$project_dir/scripts/spenblet" /usr/local/bin/spenblet
printf '%s\n' 'uinput' | sudo tee /etc/modules-load.d/spenblet.conf >/dev/null

if adb get-state >/dev/null 2>&1; then
    apk_path=${TMPDIR:-/tmp}/spenblet-1.0.0-beta.1.apk
    curl -fL "$release_url" -o "$apk_path"
    adb install -r "$apk_path"
    rm -f "$apk_path"
else
    printf '%s\n' 'APK installation skipped: connect and authorize the phone, then run:' >&2
    printf '%s\n' 'adb install -r spenblet-1.0.0-beta.1.apk' >&2
fi

printf '%s\n' 'Installation complete. Open spenblet on the phone, then run:'
printf '%s\n' 'spenblet'

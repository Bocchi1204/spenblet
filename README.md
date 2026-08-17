# spenblet

Use a Samsung S Pen phone as a wired drawing tablet on Linux.

spenblet consists of an Android drawing surface and a Linux daemon. They communicate only through a USB ADB port forward. The daemon creates a standard virtual pen device through `uinput`, allowing applications such as Krita to consume its input through libinput on Wayland or X11.

## Status

Beta 1.0 has been validated with a Galaxy S24 Ultra, CachyOS, KDE Wayland, and Krita. The Android app captures stylus input on a dark, non-drawing tablet surface and streams normalized coordinates, pressure, hover distance, tilt, contact, and stylus-button state. The Linux daemon accepts that stream and emits a virtual pen device.

## Requirements

- A Samsung Galaxy Note or S device whose Android input stack exposes S Pen events
- USB debugging enabled and an authorized ADB connection
- Linux with the `uinput` kernel module and permission to open `/dev/uinput`
- Android Platform Tools (`adb`)

## Install Beta 1.0

Install `adb` and the tools needed to build the Linux daemon:

```sh
# Arch, CachyOS, Manjaro
sudo pacman -S --needed android-tools android-udev rust

# Debian, Ubuntu
sudo apt install adb cargo
```

Download and install the APK from the GitHub release. Enable USB debugging on the phone, connect it, accept the authorization prompt, and verify the connection:

```sh
adb devices
```

Build the Linux daemon and install the `uinput` access rule:

```sh
cargo build --manifest-path linux/Cargo.toml --release
sudo ./scripts/install-udev.sh
```

Log out and back in if access to `/dev/uinput` is denied. Open the Android app, then run:

```sh
adb forward tcp:27183 tcp:27183
sudo modprobe uinput
./linux/target/release/spenblet-daemon
```

In Krita, select the `spenblet Pen` tablet device if needed.

On KDE Wayland, start Krita natively if the distribution defaults to XWayland:

```sh
QT_QPA_PLATFORM=wayland krita
```

The ADB forward listens only on `127.0.0.1:27183`; the daemon connects to that local endpoint and is not a network service.

## Licensing

spenblet is licensed under GPL-3.0-or-later. See [LICENSE](LICENSE).

## Repository layout

- `android/`: Android companion application
- `linux/`: Rust daemon for Linux input injection
- `packaging/`: distribution integration files
- `docs/`: protocol and development documentation

## Build checks

```sh
cargo fmt --manifest-path linux/Cargo.toml -- --check
cargo clippy --manifest-path linux/Cargo.toml -- -D warnings
cargo test --manifest-path linux/Cargo.toml
cd android
./gradlew assembleDebug lint
```

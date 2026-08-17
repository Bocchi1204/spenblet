# spenblet

Use a Samsung S Pen phone as a wired drawing tablet on Linux.

spenblet consists of an Android drawing surface and a Linux daemon. They communicate only through a USB ADB port forward. The daemon creates a standard virtual pen device through `uinput`, allowing applications such as Krita to consume its input through libinput on Wayland or X11.

## Requirements

- A Samsung Galaxy Note or S device whose Android input stack exposes S Pen events
- USB debugging enabled and an authorized ADB connection
- Linux with the `uinput` kernel module and permission to open `/dev/uinput`
- Android Platform Tools (`adb`)

## Install Beta 1.0

For Arch-based, Debian, and Ubuntu-based distributions, connect the phone with USB debugging enabled, accept the authorization prompt, and run:

```sh
git clone https://github.com/Bocchi1204/spenblet.git && cd spenblet && ./scripts/oneliner.sh
```

The installer detects the package manager, installs the required tools, builds the daemon, installs the `uinput` rule, and installs the APK when an authorized device is available. It does not replace the manual commands below, which are useful for reviewing each step.

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
sudo modprobe uinput
sudo ./scripts/install-udev.sh
sudo install -Dm755 linux/target/release/spenblet-daemon /usr/local/bin/spenblet-daemon
sudo install -Dm755 scripts/spenblet /usr/local/bin/spenblet
printf '%s\n' 'uinput' | sudo tee /etc/modules-load.d/spenblet.conf >/dev/null
```

Log out and back in if access to `/dev/uinput` is denied. Open the Android app, then run:

```sh
spenblet
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

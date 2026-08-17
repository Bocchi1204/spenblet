# CachyOS test procedure

Install the development dependencies:

```sh
sudo pacman -S android-tools rust
sudo modprobe uinput
```

Install the supplied udev rule, then log out and back in so the active user can open `/dev/uinput`:

```sh
sudo install -Dm644 packaging/udev/99-spenblet-uinput.rules /etc/udev/rules.d/99-spenblet-uinput.rules
sudo udevadm control --reload-rules
sudo udevadm trigger --name-match=uinput
```

Build the daemon and create the USB tunnel:

```sh
cargo build --manifest-path linux/Cargo.toml --release
adb forward tcp:27183 tcp:27183
./linux/target/release/spenblet-daemon
```

Verify that Linux sees `spenblet Pen`:

```sh
libinput list-devices
```

Run the Android application, draw with the S Pen, and test pressure in Krita. If the device does not appear in Krita, capture `libinput list-devices` output and Krita's tablet settings.

On a KDE Wayland session, verify that Krita uses its native Wayland backend:

```sh
QT_QPA_PLATFORM=wayland krita
```

An existing XWayland instance must be closed before running this command.

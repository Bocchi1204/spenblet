# Beta 1.0

The first public beta provides:

- Samsung S Pen input through a USB ADB connection
- Absolute position, 4096 pressure levels, hover, distance, tilt, contact, and side-button events
- A standard Linux `uinput` tablet recognized by libinput
- KDE Wayland and Krita compatibility
- Automatic reconnection after transport interruptions
- A full-screen Android tablet surface with a dark dotted background and no local stroke rendering

Validated hardware and software:

- Samsung Galaxy S24 Ultra
- CachyOS with KDE Wayland
- Krita 6

Known limitations:

- Linux installation and ADB forwarding are command-line operations
- Display selection and calibration are not configurable yet
- Debian and Ubuntu packaging is not available yet

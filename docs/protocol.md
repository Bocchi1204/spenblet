# Input protocol

The Android application serves a TCP socket on device port `27183`. ADB forwards host-local port `27183` to it, and the Linux daemon connects to the forwarded endpoint. Each input sample is one UTF-8, newline-terminated record:

```text
SPENBLET/1 <kind> <x> <y> <pressure> <distance> <tilt_x> <tilt_y> <button>
```

`kind` is `down`, `move`, `hover`, or `up`. Coordinates are normalized to the inclusive range `0..=65535`; pressure is `0..=4095`; distance is `0..=255`; tilt axes are signed `-9000..=9000` in centidegrees; button is `0` or `1`. Android reports tilt magnitude and orientation, and the companion application resolves those into X/Y axes before sending them.

Records are deliberately simple so the stream can be inspected with ordinary tools. Future protocol changes must use a new version identifier.

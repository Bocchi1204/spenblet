use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::os::fd::AsRawFd;
use std::thread;
use std::time::Duration;

const PORT: u16 = 27183;
const MAX_COORDINATE: i32 = 65_535;
const MAX_PRESSURE: i32 = 4_095;
const MAX_DISTANCE: i32 = 255;
const MAX_TILT: i32 = 9_000;
const X_RESOLUTION: i32 = 400;
const Y_RESOLUTION: i32 = 867;

const EV_SYN: u16 = 0x00;
const EV_KEY: u16 = 0x01;
const EV_ABS: u16 = 0x03;
const SYN_REPORT: u16 = 0;
const BTN_TOOL_PEN: u16 = 0x140;
const BTN_STYLUS: u16 = 0x14b;
const BTN_TOUCH: u16 = 0x14a;
const ABS_X: u16 = 0x00;
const ABS_Y: u16 = 0x01;
const ABS_PRESSURE: u16 = 0x18;
const ABS_DISTANCE: u16 = 0x19;
const ABS_TILT_X: u16 = 0x1a;
const ABS_TILT_Y: u16 = 0x1b;

#[repr(C)]
struct InputId {
    bus_type: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

#[repr(C)]
struct UinputSetup {
    id: InputId,
    name: [u8; 80],
    ff_effects_max: u32,
}

#[repr(C)]
struct InputAbsInfo {
    value: i32,
    minimum: i32,
    maximum: i32,
    fuzz: i32,
    flat: i32,
    resolution: i32,
}

#[repr(C)]
struct UinputAbsSetup {
    code: u16,
    padding: u16,
    absinfo: InputAbsInfo,
}

#[repr(C)]
struct InputEvent {
    time: libc::timeval,
    event_type: u16,
    code: u16,
    value: i32,
}

struct Tablet {
    device: std::fs::File,
}

impl Tablet {
    fn create() -> io::Result<Self> {
        let device = OpenOptions::new().write(true).open("/dev/uinput")?;
        let fd = device.as_raw_fd();

        for event_type in [EV_KEY, EV_ABS] {
            ioctl(fd, ui_set_evbit(), event_type as libc::c_ulong)?;
        }
        for key in [BTN_TOOL_PEN, BTN_TOUCH, BTN_STYLUS] {
            ioctl(fd, ui_set_keybit(), key as libc::c_ulong)?;
        }
        for axis in [
            ABS_X,
            ABS_Y,
            ABS_PRESSURE,
            ABS_DISTANCE,
            ABS_TILT_X,
            ABS_TILT_Y,
        ] {
            ioctl(fd, ui_set_absbit(), axis as libc::c_ulong)?;
        }

        let mut setup = UinputSetup {
            id: InputId {
                bus_type: 0x03,
                vendor: 0x04e8,
                product: 0x7370,
                version: 1,
            },
            name: [0; 80],
            ff_effects_max: 0,
        };
        setup.name[..12].copy_from_slice(b"spenblet Pen");
        ioctl_ptr(fd, ui_dev_setup(), &setup)?;

        for (axis, maximum) in [
            (ABS_X, MAX_COORDINATE),
            (ABS_Y, MAX_COORDINATE),
            (ABS_PRESSURE, MAX_PRESSURE),
            (ABS_DISTANCE, MAX_DISTANCE),
            (ABS_TILT_X, MAX_TILT),
            (ABS_TILT_Y, MAX_TILT),
        ] {
            let setup = UinputAbsSetup {
                code: axis,
                padding: 0,
                absinfo: InputAbsInfo {
                    value: 0,
                    minimum: if axis == ABS_TILT_X || axis == ABS_TILT_Y {
                        -maximum
                    } else {
                        0
                    },
                    maximum,
                    fuzz: 0,
                    flat: 0,
                    resolution: match axis {
                        ABS_X => X_RESOLUTION,
                        ABS_Y => Y_RESOLUTION,
                        _ => 0,
                    },
                },
            };
            ioctl_ptr(fd, ui_abs_setup(), &setup)?;
        }
        ioctl(fd, ui_dev_create(), 0)?;
        Ok(Self { device })
    }

    fn event(&mut self, event_type: u16, code: u16, value: i32) -> io::Result<()> {
        let event = InputEvent {
            time: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            event_type,
            code,
            value,
        };
        let bytes = unsafe {
            std::slice::from_raw_parts(
                (&event as *const InputEvent).cast(),
                std::mem::size_of::<InputEvent>(),
            )
        };
        self.device.write_all(bytes)
    }

    fn sync(&mut self) -> io::Result<()> {
        self.event(EV_SYN, SYN_REPORT, 0)
    }
}

impl Drop for Tablet {
    fn drop(&mut self) {
        unsafe { libc::ioctl(self.device.as_raw_fd(), ui_dev_destroy(), 0) };
    }
}

fn ioctl(fd: i32, request: libc::c_ulong, value: libc::c_ulong) -> io::Result<()> {
    if unsafe { libc::ioctl(fd, request, value) } == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn ioctl_ptr<T>(fd: i32, request: libc::c_ulong, value: &T) -> io::Result<()> {
    if unsafe { libc::ioctl(fd, request, value) } == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

const fn iow(nr: u32, size: u32) -> libc::c_ulong {
    (0x4000_0000u64 | ((size as u64) << 16) | (0x55u64 << 8) | nr as u64) as libc::c_ulong
}
const fn ui_set_evbit() -> libc::c_ulong {
    iow(100, 4)
}
const fn ui_set_keybit() -> libc::c_ulong {
    iow(101, 4)
}
const fn ui_set_absbit() -> libc::c_ulong {
    iow(103, 4)
}
const fn ui_dev_create() -> libc::c_ulong {
    0x5501
}
const fn ui_dev_destroy() -> libc::c_ulong {
    0x5502
}
const fn ui_dev_setup() -> libc::c_ulong {
    iow(3, std::mem::size_of::<UinputSetup>() as u32)
}
const fn ui_abs_setup() -> libc::c_ulong {
    iow(4, std::mem::size_of::<UinputAbsSetup>() as u32)
}

fn handle_client(stream: TcpStream, tablet: &mut Tablet) -> io::Result<()> {
    for line in BufReader::new(stream).lines() {
        let line = line?;
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 9 || fields[0] != "SPENBLET/1" {
            continue;
        }
        let values: Result<Vec<i32>, _> = fields[2..8].iter().map(|value| value.parse()).collect();
        let Ok(values) = values else {
            continue;
        };
        let button = match fields[8] {
            "0" => 0,
            "1" => 1,
            _ => continue,
        };
        tablet.event(EV_ABS, ABS_X, values[0].clamp(0, MAX_COORDINATE))?;
        tablet.event(EV_ABS, ABS_Y, values[1].clamp(0, MAX_COORDINATE))?;
        tablet.event(EV_ABS, ABS_PRESSURE, values[2].clamp(0, MAX_PRESSURE))?;
        tablet.event(EV_ABS, ABS_DISTANCE, values[3].clamp(0, MAX_DISTANCE))?;
        tablet.event(EV_ABS, ABS_TILT_X, values[4].clamp(-MAX_TILT, MAX_TILT))?;
        tablet.event(EV_ABS, ABS_TILT_Y, values[5].clamp(-MAX_TILT, MAX_TILT))?;
        let touching = i32::from(fields[1] != "hover" && fields[1] != "up");
        tablet.event(EV_KEY, BTN_TOOL_PEN, 1)?;
        tablet.event(EV_KEY, BTN_TOUCH, touching)?;
        tablet.event(EV_KEY, BTN_STYLUS, button)?;
        tablet.sync()?;
    }
    tablet.event(EV_KEY, BTN_TOUCH, 0)?;
    tablet.event(EV_KEY, BTN_STYLUS, 0)?;
    tablet.event(EV_KEY, BTN_TOOL_PEN, 0)?;
    tablet.sync()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut tablet = Tablet::create()?;
    loop {
        match TcpStream::connect(("127.0.0.1", PORT)) {
            Ok(stream) => {
                eprintln!("spenblet: connected to Android companion");
                if let Err(error) = handle_client(stream, &mut tablet) {
                    eprintln!("spenblet: connection error: {error}");
                }
            }
            Err(error) => eprintln!("spenblet: waiting for Android companion: {error}"),
        }
        thread::sleep(Duration::from_secs(2));
    }
}

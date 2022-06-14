use std::io::Read;
use std::os::unix::net::UnixStream;

extern crate quickrandr;
use quickrandr::cmd_profile;

const ACPID_SOCKET: &str = "/var/run/acpid.socket";

const LID_OPEN: &[u8] = "button/lid LID open".as_bytes();
const LID_CLOSE: &[u8] = "button/lid LID close".as_bytes();
const DOCK: &[u8] = "ibm/hotkey LEN0068:00 00000080 00004010".as_bytes();
const UNDOCK: &[u8] = "ibm/hotkey LEN0068:00 00000080 00004011".as_bytes();

#[derive(Debug, PartialEq, Eq)]
enum AcpidEvent {
    Unknown,
    LidOpen,
    LidClose,
    Docked,
    Undocked,
}

fn handle_event(event: AcpidEvent) {
    match event {
        AcpidEvent::Docked => {
            // The screen doesn't seem to be available right at the "dock" event
            std::thread::sleep(std::time::Duration::from_secs(3));
            let config = quickrandr::xdg_config_file().unwrap();
            cmd_profile(&config, "docked", false)
        }
        AcpidEvent::Undocked => {
            let config = quickrandr::xdg_config_file().unwrap();
            cmd_profile(&config, "default", false)
        }
        _ => { /* Other events not handled */ }
    }
}

fn run_daemon() -> Result<(), String> {
    let mut stream = match UnixStream::connect(ACPID_SOCKET) {
        Ok(sock) => sock,
        Err(e) => return Err(format!("Failed to connect to acpid socket: {}", e)),
    };

    let mut byte = vec![0; 1];
    let mut buffer = vec![0; 128];
    loop {
        stream.read_exact(&mut byte).unwrap();

        // Events are separated by newline characters
        if byte[0] == 0x0a {
            buffer.clear();
            continue;
        }

        buffer.push(byte[0]);

        let event = match buffer.as_slice() {
            LID_OPEN => AcpidEvent::LidOpen,
            LID_CLOSE => AcpidEvent::LidClose,
            DOCK => AcpidEvent::Docked,
            UNDOCK => AcpidEvent::Undocked,
            _ => AcpidEvent::Unknown,
        };

        if event != AcpidEvent::Unknown {
            std::thread::spawn(move || {
                handle_event(event);
            });
        }
    }
}

fn main() {
    std::process::exit(match run_daemon() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {err:?}");
            1
        }
    });
}

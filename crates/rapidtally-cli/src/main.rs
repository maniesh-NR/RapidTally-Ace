//! `rapidtally-ace` — command-line front end for the RapidTally Ace
//! clean-room QuickBooks Desktop format probe (project phase 0).
//!
//! Exit codes:
//! * `0` — success
//! * `2` — usage error
//! * `3` — I/O error
//! * `4` — input exceeds the phase-0 size cap

use rapidtally_format::probe::probe;
use rapidtally_format::VERSION;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = concat!(
    "rapidtally-ace ",
    env!("CARGO_PKG_VERSION"),
    " - clean-room QuickBooks Desktop format probe (phase 0)

USAGE:
    rapidtally-ace inspect <file>    Probe a .QBW/.QBB file and print a structural report
    rapidtally-ace probe <file>      Alias for `inspect`
    rapidtally-ace --version         Print version
    rapidtally-ace --help            Print this help

EXIT CODES:
    0  success
    2  usage error
    3  I/O error
    4  input exceeds the phase-0 size cap
"
);

/// Whole-file read cap: the phase-0 probe is a spike that loads the entire
/// file into memory. 2 GiB on 64-bit hosts, 512 MiB on 32-bit hosts.
const MAX_INPUT_64BIT: u64 = 2 * 1024 * 1024 * 1024;
const MAX_INPUT_32BIT: u64 = 512 * 1024 * 1024;

fn main() -> ExitCode {
    let owned: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = owned.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version" | "-V"] => {
            println!("rapidtally-ace {VERSION}");
            ExitCode::SUCCESS
        }
        ["--help" | "-h"] => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        ["inspect" | "probe", path] => run(Path::new(path)),
        _ => {
            eprint!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run(path: &Path) -> ExitCode {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: cannot open {}: {e}", path.display());
            return ExitCode::from(3);
        }
    };
    let cap = if cfg!(target_pointer_width = "64") {
        MAX_INPUT_64BIT
    } else {
        MAX_INPUT_32BIT
    };
    if meta.len() > cap {
        eprintln!(
            "error: file is {} bytes; the phase-0 probe loads the whole file and caps input at {cap} bytes",
            meta.len()
        );
        return ExitCode::from(4);
    }
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", path.display());
            return ExitCode::from(3);
        }
    };

    let report = probe(&data);
    println!("RapidTally Ace {VERSION} - clean-room format probe (phase 0)");
    println!("file: {}", path.display());
    println!("{report}");
    ExitCode::SUCCESS
}

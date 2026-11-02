//! Classic 16-bytes-per-row hex dumps for manual inspection.

use std::fmt::Write as _;

/// Renders `data` as a canonical hex dump: one row per 16 bytes, with the
/// offset, the hex bytes (space-padded) and the printable ASCII gutter.
///
/// Non-printable bytes appear as `.` in the ASCII gutter.
pub fn hexdump(data: &[u8]) -> String {
    let mut out = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        let _ = write!(out, "{:08x}  ", i * 16);
        for byte in chunk {
            let _ = write!(out, "{:02x} ", byte);
        }
        if chunk.len() < 16 {
            for _ in chunk.len()..16 {
                out.push_str("   ");
            }
        }
        out.push_str(" |");
        for &b in chunk {
            out.push(if (0x20..0x7f).contains(&b) {
                b as char
            } else {
                '.'
            });
        }
        out.push_str("|\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_row() {
        let dump = hexdump(b"ABCDEFGHIJKLMNOP");
        let lines: Vec<&str> = dump.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].starts_with("00000000  41 42 43 44"));
        assert!(lines[0].ends_with("|ABCDEFGHIJKLMNOP|"));
    }

    #[test]
    fn rows_and_padding() {
        let dump = hexdump(b"ABCDEFGHIJ");
        let lines: Vec<&str> = dump.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("41 42 43 44 45 46 47 48 49 4a"));
        assert!(lines[0].ends_with("|ABCDEFGHIJ|"));
    }

    #[test]
    fn second_row_offset() {
        let data = [0u8; 20];
        let dump = hexdump(&data);
        let lines: Vec<&str> = dump.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].starts_with("00000010"));
    }

    #[test]
    fn empty_input() {
        assert_eq!(hexdump(&[]), "");
    }
}

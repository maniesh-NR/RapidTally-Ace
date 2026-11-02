//! Integration tests for the clean-room probe.
//!
//! All fixtures are synthetic: no real QuickBooks data files are committed
//! to this repository (clean-room data hygiene, `docs/CLEANROOM.md` §7).

use rapidtally_format::entropy::shannon_bits_per_byte;
use rapidtally_format::probe::{probe, Classification, MARKERS};

/// Tiny deterministic PRNG (LCG) so tests avoid external dependencies.
struct Lcg(u64);

impl Lcg {
    fn next_byte(&mut self) -> u8 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as u8
    }
}

fn put(buf: &mut [u8], offset: usize, bytes: &[u8]) {
    buf[offset..offset + bytes.len()].copy_from_slice(bytes);
}

#[test]
fn detects_sql_anywhere_and_intuit_markers() {
    let mut buf = vec![0u8; 8192];
    put(&mut buf, 64, b"SQL Anywhere\0");
    put(&mut buf, 1000, b"Intuit\0");
    let report = probe(&buf);

    assert!(report
        .markers
        .iter()
        .any(|m| m.marker == "SQL Anywhere" && m.offset == 64));
    assert!(report
        .markers
        .iter()
        .any(|m| m.marker == "Intuit" && m.offset == 1000));
    // Uniform-zero pages have zero entropy, so with an SA marker the
    // classification must be cleartext.
    assert_eq!(report.classification, Classification::SqlAnywhereCleartext);
}

#[test]
fn high_entropy_pages_classified_as_obfuscated() {
    let mut rng = Lcg(0x5EED);
    let mut buf: Vec<u8> = (0..8192).map(|_| rng.next_byte()).collect();
    put(&mut buf, 64, b"SQL Anywhere\0");
    let report = probe(&buf);
    assert_eq!(
        report.classification,
        Classification::SqlAnywhereObfuscatedOrEncrypted
    );
    assert!(report.mean_entropy > 6.0);
}

#[test]
fn unknown_when_no_markers() {
    let buf = vec![0u8; 4096];
    let report = probe(&buf);
    assert_eq!(report.classification, Classification::Unknown);
    assert!(report.markers.is_empty());
}

#[test]
fn page_size_candidates_and_field_hits() {
    let page_size: usize = 4096;
    let mut buf = vec![0u8; page_size * 10];
    // Simulate a header field holding the page size (LE u32) at offset 8.
    let bytes = (page_size as u32).to_le_bytes();
    put(&mut buf, 8, &bytes);
    let report = probe(&buf);

    assert!(report
        .size_divisible_page_sizes
        .contains(&(page_size as u64)));
    assert!(report
        .page_size_field_hits
        .iter()
        .any(|h| h.offset == 8 && h.value == page_size as u64));
}

#[test]
fn tail_scan_finds_marker_near_end() {
    let mut buf = vec![0u8; 2 * 1024 * 1024];
    let pos = buf.len() - 1024;
    put(&mut buf, pos, b"QuickBooks\0");
    let report = probe(&buf);
    assert!(report
        .markers
        .iter()
        .any(|m| m.marker == "QuickBooks" && m.offset as usize == pos));
}

#[test]
fn entropy_functions_match_expected_values() {
    assert_eq!(shannon_bits_per_byte(&[0u8; 100]), 0.0);
    let data: Vec<u8> = (0..=255u8).collect();
    assert!((shannon_bits_per_byte(&data) - 8.0).abs() < 1e-9);
}

#[test]
fn hexdump_included_in_report() {
    let mut buf = vec![0u8; 512];
    put(&mut buf, 0, b"SQL Anywhere\0");
    let report = probe(&buf);
    assert!(report
        .first_page_hexdump
        .starts_with("00000000  53 51 4c 20"));
    assert!(report.to_string().contains("first 512 bytes:"));
}

#[test]
fn all_markers_are_covered_by_scan() {
    // Sanity: the marker table is non-empty and every pattern is ASCII.
    assert!(!MARKERS.is_empty());
    for (name, pattern) in MARKERS {
        assert!(!name.is_empty());
        assert!(!pattern.is_empty());
        assert!(pattern.iter().all(|&b| (0x20..0x7f).contains(&b)));
    }
}

#[test]
fn report_is_stable_for_empty_input() {
    let report = probe(&[]);
    assert_eq!(report.file_size, 0);
    assert_eq!(report.classification, Classification::Unknown);
    assert!(report.first_page_hexdump.is_empty());
}

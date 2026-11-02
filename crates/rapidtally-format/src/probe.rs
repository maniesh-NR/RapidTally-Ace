//! Clean-room structural probe for QuickBooks Desktop database files
//! (`.QBW` / `.QBB`).
//!
//! Everything in this module is derived from black-box heuristics and
//! publicly available documentation about SAP SQL Anywhere database files.
//! No Intuit source code, SDK material or decompiled output was used or is
//! referenced here (see `docs/CLEANROOM.md` in the repository).
//!
//! The probe reports *hypotheses with evidence*, not a verified parse:
//!
//! 1. **ASCII markers** — signatures such as `SQL Anywhere`, `Adaptive
//!    Server`, `Sybase`, `Intuit` and `QuickBooks` are searched in the first
//!    1 MiB and the last 64 KiB of the file.
//! 2. **Page-size candidates** — SQL Anywhere databases are page-structured;
//!    the probe reports which common page sizes divide the file size evenly,
//!    plus every 4-byte-aligned little-endian `u32` in the first 4 KiB that
//!    equals a common page size (header field hypothesis).
//! 3. **Entropy map** — the file is split into 16 regions and each region's
//!    Shannon entropy is measured. High entropy is consistent with Intuit's
//!    page-level obfuscation/encryption layer; low entropy suggests cleartext
//!    pages suitable for catalog extraction (G1 work).
//! 4. **Hex dump** — the first 512 bytes for manual inspection.

use crate::entropy::shannon_bits_per_byte;
use crate::hexdump::hexdump;
use std::fmt;

/// ASCII signatures probed in the file head/tail.
pub const MARKERS: &[(&str, &[u8])] = &[
    ("SQL Anywhere", b"SQL Anywhere"),
    ("Adaptive Server", b"Adaptive Server"),
    ("Sybase", b"Sybase"),
    ("Intuit", b"Intuit"),
    ("QuickBooks", b"QuickBooks"),
];

/// Page sizes used by SAP SQL Anywhere builds commonly seen in the wild.
pub const CANDIDATE_PAGE_SIZES: &[u64] = &[1024, 2048, 4096, 8192, 16384, 32768, 65536];

/// Number of entropy regions the file is split into.
pub const ENTROPY_REGIONS: usize = 16;

/// Bytes hex-dumped at the front of the report.
pub const HEXDUMP_BYTES: usize = 512;

const SCAN_HEAD: usize = 1024 * 1024;
const SCAN_TAIL: usize = 64 * 1024;
const MAX_HITS_PER_MARKER: usize = 4;
const MAX_FIELD_HITS_PER_VALUE: usize = 4;
const MIN_PAGES_FOR_CANDIDATE: u64 = 8;

/// A single ASCII signature hit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerHit {
    /// The marker name, e.g. `"SQL Anywhere"`.
    pub marker: &'static str,
    /// Byte offset of the hit.
    pub offset: u64,
}

/// A candidate page-size value found embedded in the file header area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldHit {
    /// Byte offset of the little-endian `u32` value.
    pub offset: u64,
    /// The value found.
    pub value: u64,
}

/// One entropy measurement over a contiguous region.
#[derive(Debug, Clone)]
pub struct EntropyRegion {
    /// First byte of the region.
    pub start: u64,
    /// Region length in bytes.
    pub len: u64,
    /// Shannon entropy of the region in bits per byte (0.0 – 8.0).
    pub bits_per_byte: f64,
}

/// Heuristic classification of the probed file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    /// SAP SQL Anywhere markers present and pages look cleartext.
    SqlAnywhereCleartext,
    /// SAP SQL Anywhere markers present and pages look obfuscated or
    /// encrypted (consistent with Intuit's page-level layer).
    SqlAnywhereObfuscatedOrEncrypted,
    /// Intuit/QuickBooks markers present but no SQL Anywhere signature.
    IntuitSignatureOnly,
    /// Nothing recognized.
    Unknown,
}

impl Classification {
    /// Human-readable label (ASCII only, safe for terminals).
    pub fn label(self) -> &'static str {
        match self {
            Classification::SqlAnywhereCleartext => {
                "SQL Anywhere database - cleartext pages (low confidence)"
            }
            Classification::SqlAnywhereObfuscatedOrEncrypted => {
                "SQL Anywhere database - obfuscated or encrypted pages (low confidence)"
            }
            Classification::IntuitSignatureOnly => {
                "Intuit/QuickBooks signature found, SQL Anywhere signature not found (low confidence)"
            }
            Classification::Unknown => "unknown format (low confidence)",
        }
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// Full result of a probe run.
#[derive(Debug, Clone)]
pub struct ProbeReport {
    /// Input size in bytes.
    pub file_size: u64,
    /// ASCII signature hits (head + tail scan).
    pub markers: Vec<MarkerHit>,
    /// Candidate page sizes that divide the file size evenly (≥ 8 pages).
    pub size_divisible_page_sizes: Vec<u64>,
    /// Little-endian `u32` header-field hits equal to a candidate page size.
    pub page_size_field_hits: Vec<FieldHit>,
    /// Entropy measurements, one per region.
    pub entropy_regions: Vec<EntropyRegion>,
    /// Length-weighted mean entropy across all regions.
    pub mean_entropy: f64,
    /// Heuristic classification.
    pub classification: Classification,
    /// Explanation notes attached to the classification.
    pub notes: Vec<String>,
    /// Hex dump of the first [`HEXDUMP_BYTES`] bytes.
    pub first_page_hexdump: String,
}

/// Probes an in-memory buffer and returns a structural report.
pub fn probe(data: &[u8]) -> ProbeReport {
    let file_size = data.len() as u64;
    let markers = scan_markers(data);
    let size_divisible_page_sizes = scan_size_divisible_page_sizes(file_size);
    let page_size_field_hits = scan_page_size_fields(data);
    let entropy_regions = scan_entropy_regions(data);
    let mean_entropy = weighted_mean_entropy(&entropy_regions);
    let classification = classify(&markers, mean_entropy);
    let notes = build_notes(&classification, mean_entropy, &size_divisible_page_sizes);
    let first_page_hexdump = hexdump(&data[..data.len().min(HEXDUMP_BYTES)]);

    ProbeReport {
        file_size,
        markers,
        size_divisible_page_sizes,
        page_size_field_hits,
        entropy_regions,
        mean_entropy,
        classification,
        notes,
        first_page_hexdump,
    }
}

fn find_all(haystack: &[u8], needle: &[u8], limit: usize) -> Vec<u64> {
    let mut hits = Vec::new();
    if needle.is_empty() || haystack.len() < needle.len() {
        return hits;
    }
    let max_start = haystack.len() - needle.len();
    let mut start = 0usize;
    while start <= max_start && hits.len() < limit {
        if &haystack[start..start + needle.len()] == needle {
            hits.push(start as u64);
            start += needle.len();
        } else {
            start += 1;
        }
    }
    hits
}

fn scan_markers(data: &[u8]) -> Vec<MarkerHit> {
    let mut hits = Vec::new();
    let head_len = data.len().min(SCAN_HEAD);
    let head = &data[..head_len];
    let tail_start = data.len().saturating_sub(SCAN_TAIL);
    let tail = if tail_start > head_len {
        &data[tail_start..]
    } else {
        &[]
    };

    for (name, pattern) in MARKERS {
        for offset in find_all(head, pattern, MAX_HITS_PER_MARKER) {
            hits.push(MarkerHit {
                marker: name,
                offset,
            });
        }
        for offset in find_all(tail, pattern, 2) {
            hits.push(MarkerHit {
                marker: name,
                offset: tail_start as u64 + offset,
            });
        }
    }
    hits.sort_by_key(|h| h.offset);
    hits
}

fn scan_size_divisible_page_sizes(file_size: u64) -> Vec<u64> {
    CANDIDATE_PAGE_SIZES
        .iter()
        .copied()
        .filter(|&ps| file_size % ps == 0 && file_size / ps >= MIN_PAGES_FOR_CANDIDATE)
        .collect()
}

fn scan_page_size_fields(data: &[u8]) -> Vec<FieldHit> {
    let mut hits = Vec::new();
    let scan_len = data.len().min(4096);
    if scan_len < 4 {
        return hits;
    }
    for &candidate in CANDIDATE_PAGE_SIZES {
        let mut found = 0usize;
        let mut offset = 0usize;
        while offset + 4 <= scan_len && found < MAX_FIELD_HITS_PER_VALUE {
            let value = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            if value as u64 == candidate {
                hits.push(FieldHit {
                    offset: offset as u64,
                    value: candidate,
                });
                found += 1;
                offset += 4;
            } else {
                offset += 4;
            }
        }
    }
    hits.sort_by_key(|h| h.offset);
    hits
}

fn scan_entropy_regions(data: &[u8]) -> Vec<EntropyRegion> {
    let mut regions = Vec::new();
    if data.is_empty() {
        return regions;
    }
    let region_len = data.len() / ENTROPY_REGIONS;
    if region_len == 0 {
        regions.push(EntropyRegion {
            start: 0,
            len: data.len() as u64,
            bits_per_byte: shannon_bits_per_byte(data),
        });
        return regions;
    }
    for i in 0..ENTROPY_REGIONS {
        let start = i * region_len;
        let end = if i == ENTROPY_REGIONS - 1 {
            data.len()
        } else {
            start + region_len
        };
        regions.push(EntropyRegion {
            start: start as u64,
            len: (end - start) as u64,
            bits_per_byte: shannon_bits_per_byte(&data[start..end]),
        });
    }
    regions
}

fn weighted_mean_entropy(regions: &[EntropyRegion]) -> f64 {
    let total: u64 = regions.iter().map(|r| r.len).sum();
    if total == 0 {
        return 0.0;
    }
    let acc: f64 = regions.iter().map(|r| r.bits_per_byte * r.len as f64).sum();
    acc / total as f64
}

fn classify(markers: &[MarkerHit], mean_entropy: f64) -> Classification {
    let sa_marker = markers.iter().any(|m| {
        m.marker == "SQL Anywhere" || m.marker == "Adaptive Server" || m.marker == "Sybase"
    });
    let intuit_marker = markers
        .iter()
        .any(|m| m.marker == "Intuit" || m.marker == "QuickBooks");
    if sa_marker && mean_entropy < 6.0 {
        Classification::SqlAnywhereCleartext
    } else if sa_marker {
        Classification::SqlAnywhereObfuscatedOrEncrypted
    } else if intuit_marker {
        Classification::IntuitSignatureOnly
    } else {
        Classification::Unknown
    }
}

fn build_notes(
    classification: &Classification,
    mean_entropy: f64,
    page_sizes: &[u64],
) -> Vec<String> {
    let mut notes = vec![
        "Phase-0 heuristic probe: classification is a hypothesis with evidence, not a verified parse.".to_string(),
        "ASCII marker scan covers the first 1 MiB and the last 64 KiB of the file.".to_string(),
    ];
    match classification {
        Classification::SqlAnywhereCleartext => notes.push(format!(
            "Low mean entropy ({mean_entropy:.2} bits/byte) with SAP SQL Anywhere markers suggests cleartext pages; catalog extraction is G1 work."
        )),
        Classification::SqlAnywhereObfuscatedOrEncrypted => notes.push(format!(
            "High mean entropy ({mean_entropy:.2} bits/byte) is consistent with Intuit's page-level obfuscation layer; compression and encryption cannot be distinguished yet (G1 work)."
        )),
        Classification::IntuitSignatureOnly => notes.push(
            "Intuit signatures found but no SAP SQL Anywhere markers; the container type needs ground-truth comparison (G1 work).".to_string(),
        ),
        Classification::Unknown => notes.push(
            "No known signatures found. If this is a legally obtained QuickBooks file, record it in the evidence log and compare with ground-truth samples.".to_string(),
        ),
    }
    if page_sizes.is_empty() {
        notes.push("File size is not divisible by any common SQL Anywhere page size.".to_string());
    } else {
        notes.push(format!(
            "Candidate page sizes (file-size divisibility): {}.",
            page_sizes
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    notes.push(
        "No real QuickBooks data files are included in this repository; validate against legally obtained samples only.".to_string(),
    );
    notes
}

impl fmt::Display for ProbeReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "file size            : {} bytes", self.file_size)?;
        writeln!(f, "classification       : {}", self.classification)?;
        if self.markers.is_empty() {
            writeln!(f, "markers              : none")?;
        } else {
            writeln!(f, "markers              :")?;
            for m in &self.markers {
                writeln!(f, "  - \"{}\" @ 0x{:08x}", m.marker, m.offset)?;
            }
        }
        if self.size_divisible_page_sizes.is_empty() {
            writeln!(
                f,
                "page-size candidates : none (file size not divisible by any candidate)"
            )?;
        } else {
            let list = self
                .size_divisible_page_sizes
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "page-size candidates : {list}")?;
        }
        if self.page_size_field_hits.is_empty() {
            writeln!(f, "page-size field hits : none in first 4 KiB")?;
        } else {
            writeln!(f, "page-size field hits :")?;
            for h in &self.page_size_field_hits {
                writeln!(f, "  - 0x{:08x}: {} (LE u32)", h.offset, h.value)?;
            }
        }
        writeln!(
            f,
            "entropy              : mean {:.2} bits/byte across {} regions",
            self.mean_entropy,
            self.entropy_regions.len()
        )?;
        for r in &self.entropy_regions {
            writeln!(
                f,
                "  0x{:08x}-0x{:08x} : {:.2} bits/byte",
                r.start,
                r.start + r.len,
                r.bits_per_byte
            )?;
        }
        writeln!(f, "notes                :")?;
        for n in &self.notes {
            writeln!(f, "  - {n}")?;
        }
        writeln!(
            f,
            "first {} bytes:",
            HEXDUMP_BYTES.min(self.file_size as usize)
        )?;
        write!(f, "{}", self.first_page_hexdump)
    }
}

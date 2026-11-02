//! # RapidTally Ace — clean-room QuickBooks Desktop format probe (phase 0)
//!
//! `rapidtally-format` implements black-box structural heuristics for Intuit
//! QuickBooks Desktop database files (`.QBW` / `.QBB`):
//!
//! * [`probe::MARKERS`] — ASCII signature scanning (`SQL Anywhere`, `Sybase`,
//!   `Intuit`, `QuickBooks`, ...);
//! * candidate SAP SQL Anywhere page-size analysis (file-size divisibility and
//!   a header field scan);
//! * [`entropy::shannon_bits_per_byte`] — per-region entropy mapping to
//!   distinguish cleartext pages from obfuscated/encrypted/compressed ones;
//! * [`hexdump::hexdump`] — classic hex dumps for manual inspection;
//! * [`ffi`] — a minimal C ABI so the probe can be embedded from Kotlin/JNI
//!   (Android), C++/Qt (desktop) or any other FFI-capable host.
//!
//! ## Clean-room notice
//!
//! Everything here is derived from black-box heuristics and publicly available
//! documentation. No Intuit source code, SDK material or decompiled output was
//! used or referenced. See `docs/CLEANROOM.md` in the repository.

pub mod entropy;
pub mod ffi;
pub mod hexdump;
pub mod probe;

pub use probe::{probe, Classification, MarkerHit, ProbeReport};

/// Crate version, as reported through the C ABI and the CLI.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

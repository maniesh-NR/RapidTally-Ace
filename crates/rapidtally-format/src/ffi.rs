//! Minimal C ABI so the probe can be embedded from Kotlin/JNI (Android),
//! C++/Qt (desktop) or any FFI-capable host.
//!
//! The ABI is intentionally tiny and allocation-free on the caller side:
//!
//! * [`rtace_version`] — returns a static NUL-terminated version string;
//! * [`rtace_probe_buffer`] — probes a raw buffer and writes a
//!   human-readable report into a caller-provided byte buffer.

use crate::probe::probe;
use std::os::raw::{c_char, c_int};

const VERSION_CSTR: &str = concat!("rapidtally-format ", env!("CARGO_PKG_VERSION"), "\0");

/// Returns a pointer to a static NUL-terminated version string, e.g.
/// `"rapidtally-format 0.1.0"`. The pointer stays valid for the lifetime of
/// the program; the caller must not free it.
#[no_mangle]
pub extern "C" fn rtace_version() -> *const c_char {
    VERSION_CSTR.as_ptr() as *const c_char
}

/// Probes a raw buffer and writes a human-readable report into `out`.
///
/// # Safety
///
/// * `data` must point to `len` readable bytes (or be NULL when `len == 0`);
/// * `out` must point to `out_cap` writable bytes (or be NULL);
/// * the memory regions must not overlap and must outlive the call.
///
/// Returns the **full** report length as `c_int` (may exceed `out_cap`).
/// If `out` is NULL or `out_cap` is 0, nothing is written — call once with
/// NULL to size the buffer, then again. Returns `-1` on invalid input
/// (NULL `data` with non-zero `len`). The copied payload is truncated to
/// `out_cap` bytes and is **not** NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn rtace_probe_buffer(
    data: *const u8,
    len: usize,
    out: *mut u8,
    out_cap: usize,
) -> c_int {
    if data.is_null() && len > 0 {
        return -1;
    }
    let bytes: &[u8] = if data.is_null() || len == 0 {
        &[]
    } else {
        // SAFETY: caller guarantees `len` readable bytes at `data` (see above).
        unsafe { std::slice::from_raw_parts(data, len) }
    };
    let report = probe(bytes).to_string();
    let full = report.len().min(i32::MAX as usize);
    if !out.is_null() && out_cap > 0 {
        let n = full.min(out_cap);
        // SAFETY: caller guarantees `out_cap` writable bytes at `out`;
        // the report source string is a distinct allocation, so no overlap.
        unsafe {
            std::ptr::copy_nonoverlapping(report.as_ptr(), out, n);
        }
    }
    full as c_int
}

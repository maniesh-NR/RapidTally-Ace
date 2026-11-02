//! Shannon-entropy helpers used to distinguish cleartext pages from
//! obfuscated, encrypted or compressed regions.
//!
//! Entropy is a cheap, assumption-free first signal: random-looking regions
//! (≈ 8 bits/byte) are consistent with Intuit's page-level obfuscation layer,
//! while low-entropy regions often contain plaintext catalog strings.

/// Computes the Shannon entropy of `data` in bits per byte (0.0 – 8.0).
///
/// Returns `0.0` for empty input by convention.
pub fn shannon_bits_per_byte(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0u64; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let n = data.len() as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_is_zero() {
        assert_eq!(shannon_bits_per_byte(&[]), 0.0);
    }

    #[test]
    fn uniform_input_is_zero() {
        assert_eq!(shannon_bits_per_byte(&[0u8; 1024]), 0.0);
        assert_eq!(shannon_bits_per_byte(&[0xAA; 17]), 0.0);
    }

    #[test]
    fn full_byte_range_is_eight_bits_per_byte() {
        let data: Vec<u8> = (0..=255u8).cycle().take(4096).collect();
        let e = shannon_bits_per_byte(&data);
        assert!((e - 8.0).abs() < 1e-9, "got {e}");
    }

    #[test]
    fn half_half_input_is_one_bit_per_byte() {
        let mut data = vec![0u8; 512];
        data.extend_from_slice(&[0xFF; 512]);
        let e = shannon_bits_per_byte(&data);
        assert!((e - 1.0).abs() < 1e-9, "got {e}");
    }
}

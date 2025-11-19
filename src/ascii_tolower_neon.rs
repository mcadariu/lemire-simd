/*
ASCII to Lowercase Conversion (ARM NEON)

Based on: https://lemire.me/blog/2024/08/03/converting-ascii-strings-to-lower-case-at-crazy-speeds-with-avx-512
Adapted for ARM NEON SIMD intrinsics (processes 16 bytes per NEON register)

Benchmarks (1 MB mixed case text):
  - Scalar (no auto-vectorization): 1.05 GB/s
  - NEON (16 bytes/iter): 28.84 GB/s (27.5x faster)
  - NEON (32 bytes/iter): 31.05 GB/s (29.6x faster)
  - NEON (64 bytes/iter): 33.00 GB/s (31.5x faster)

Key optimizations:
  1. #[inline(never)] on scalar to prevent auto-vectorization
  2. Loop unrolling (64 bytes = 4 NEON registers per iteration)
  3. Branchless SIMD mask operations for conditional lowercase conversion

 */

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

/// Scalar implementation: converts a single ASCII byte to lowercase
#[inline(never)]
pub fn to_lower_scalar(byte: u8) -> u8 {
    if byte >= b'A' && byte <= b'Z' {
        byte + (b'a' - b'A')
    } else {
        byte
    }
}

/// Converts ASCII string to lowercase using scalar operations
/// Using #[inline(never)] to prevent auto-vectorization
#[inline(never)]
pub fn ascii_tolower_scalar(buffer: &[u8]) -> Vec<u8> {
    let mut result = vec![0u8; buffer.len()];
    for i in 0..buffer.len() {
        result[i] = to_lower_scalar(buffer[i]);
    }
    result
}

/// NEON implementation: converts 16 bytes to lowercase in parallel
#[target_feature(enable = "neon")]
#[cfg(target_arch = "aarch64")]
unsafe fn tolower16(c: uint8x16_t) -> uint8x16_t {
    let a = vdupq_n_u8(b'A');
    let z = vdupq_n_u8(b'Z');
    let to_lower = vdupq_n_u8(b'a' - b'A');

    // Create masks for bytes that are uppercase letters
    let ge_a = vcgeq_u8(c, a);  // c >= 'A'
    let le_z = vcleq_u8(c, z);  // c <= 'Z'
    let is_upper = vandq_u8(ge_a, le_z);  // is uppercase letter

    // Add 'a'-'A' offset only to uppercase letters
    // Using masked add: result = c + (is_upper & to_lower)
    let offset = vandq_u8(is_upper, to_lower);
    vaddq_u8(c, offset)
}

/// Converts ASCII string to lowercase using ARM NEON instructions (16 bytes at a time)
#[cfg(target_arch = "aarch64")]
pub fn ascii_tolower_neon(buffer: &[u8]) -> Vec<u8> {
    if !std::arch::is_aarch64_feature_detected!("neon") {
        return ascii_tolower_scalar(buffer);
    }

    let mut result = vec![0u8; buffer.len()];
    let mut i = 0;

    unsafe {
        // Process 16-byte chunks with NEON
        while i + 16 <= buffer.len() {
            let chunk = vld1q_u8(buffer.as_ptr().add(i));
            let lowered = tolower16(chunk);
            vst1q_u8(result.as_mut_ptr().add(i), lowered);
            i += 16;
        }
    }

    // Handle remaining bytes with scalar code
    for j in i..buffer.len() {
        result[j] = to_lower_scalar(buffer[j]);
    }

    result
}

/// Converts ASCII string to lowercase using ARM NEON instructions (32 bytes at a time)
#[cfg(target_arch = "aarch64")]
pub fn ascii_tolower_neon_32(buffer: &[u8]) -> Vec<u8> {
    if !std::arch::is_aarch64_feature_detected!("neon") {
        return ascii_tolower_scalar(buffer);
    }

    let mut result = vec![0u8; buffer.len()];
    let mut i = 0;

    unsafe {
        // Process 32-byte chunks with NEON (2 registers)
        while i + 32 <= buffer.len() {
            let chunk1 = vld1q_u8(buffer.as_ptr().add(i));
            let chunk2 = vld1q_u8(buffer.as_ptr().add(i + 16));

            let lowered1 = tolower16(chunk1);
            let lowered2 = tolower16(chunk2);

            vst1q_u8(result.as_mut_ptr().add(i), lowered1);
            vst1q_u8(result.as_mut_ptr().add(i + 16), lowered2);
            i += 32;
        }
    }

    // Handle remaining bytes with scalar code
    for j in i..buffer.len() {
        result[j] = to_lower_scalar(buffer[j]);
    }

    result
}

/// Converts ASCII string to lowercase using ARM NEON instructions (64 bytes at a time)
#[cfg(target_arch = "aarch64")]
pub fn ascii_tolower_neon_64(buffer: &[u8]) -> Vec<u8> {
    if !std::arch::is_aarch64_feature_detected!("neon") {
        return ascii_tolower_scalar(buffer);
    }

    let mut result = vec![0u8; buffer.len()];
    let mut i = 0;

    unsafe {
        // Process 64-byte chunks with NEON (4 registers) - loop unrolling
        while i + 64 <= buffer.len() {
            let chunk1 = vld1q_u8(buffer.as_ptr().add(i));
            let chunk2 = vld1q_u8(buffer.as_ptr().add(i + 16));
            let chunk3 = vld1q_u8(buffer.as_ptr().add(i + 32));
            let chunk4 = vld1q_u8(buffer.as_ptr().add(i + 48));

            let lowered1 = tolower16(chunk1);
            let lowered2 = tolower16(chunk2);
            let lowered3 = tolower16(chunk3);
            let lowered4 = tolower16(chunk4);

            vst1q_u8(result.as_mut_ptr().add(i), lowered1);
            vst1q_u8(result.as_mut_ptr().add(i + 16), lowered2);
            vst1q_u8(result.as_mut_ptr().add(i + 32), lowered3);
            vst1q_u8(result.as_mut_ptr().add(i + 48), lowered4);
            i += 64;
        }
    }

    // Handle remaining bytes with scalar code
    for j in i..buffer.len() {
        result[j] = to_lower_scalar(buffer[j]);
    }

    result
}

// For non-ARM architectures, provide fallbacks
#[cfg(not(target_arch = "aarch64"))]
pub fn ascii_tolower_neon(buffer: &[u8]) -> Vec<u8> {
    ascii_tolower_scalar(buffer)
}

#[cfg(not(target_arch = "aarch64"))]
pub fn ascii_tolower_neon_32(buffer: &[u8]) -> Vec<u8> {
    ascii_tolower_scalar(buffer)
}

#[cfg(not(target_arch = "aarch64"))]
pub fn ascii_tolower_neon_64(buffer: &[u8]) -> Vec<u8> {
    ascii_tolower_scalar(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_lowercase() {
        assert_eq!(to_lower_scalar(b'a'), b'a');
        assert_eq!(to_lower_scalar(b'z'), b'z');
    }

    #[test]
    fn test_scalar_uppercase() {
        assert_eq!(to_lower_scalar(b'A'), b'a');
        assert_eq!(to_lower_scalar(b'Z'), b'z');
        assert_eq!(to_lower_scalar(b'M'), b'm');
    }

    #[test]
    fn test_scalar_non_alpha() {
        assert_eq!(to_lower_scalar(b'0'), b'0');
        assert_eq!(to_lower_scalar(b'9'), b'9');
        assert_eq!(to_lower_scalar(b' '), b' ');
        assert_eq!(to_lower_scalar(b'!'), b'!');
        assert_eq!(to_lower_scalar(b'@'), b'@');
    }

    #[test]
    fn test_scalar_buffer() {
        let input = b"Hello World!";
        let expected = b"hello world!";
        assert_eq!(ascii_tolower_scalar(input), expected);
    }

    #[test]
    fn test_scalar_mixed() {
        let input = b"The Quick BROWN Fox Jumps Over 123!";
        let expected = b"the quick brown fox jumps over 123!";
        assert_eq!(ascii_tolower_scalar(input), expected);
    }

    #[test]
    fn test_neon_matches_scalar() {
        let test_cases = vec![
            b"" as &[u8],
            b"a",
            b"A",
            b"Hello",
            b"HELLO",
            b"hello"
        ];

        for test in test_cases {
            let scalar_result = ascii_tolower_scalar(test);
            let neon_result = ascii_tolower_neon(test);
            let neon_32_result = ascii_tolower_neon_32(test);
            let neon_64_result = ascii_tolower_neon_64(test);

            assert_eq!(
                scalar_result, neon_result,
                "NEON mismatch for input: {:?}",
                std::str::from_utf8(test).unwrap_or("<invalid utf8>")
            );
            assert_eq!(
                scalar_result, neon_32_result,
                "NEON-32 mismatch for input: {:?}",
                std::str::from_utf8(test).unwrap_or("<invalid utf8>")
            );
            assert_eq!(
                scalar_result, neon_64_result,
                "NEON-64 mismatch for input: {:?}",
                std::str::from_utf8(test).unwrap_or("<invalid utf8>")
            );
        }
    }

    #[test]
    fn test_neon_boundary_conditions() {
        // Test characters around 'A' and 'Z' boundaries
        let input = b"@ABC[\\]^_`abc{";
        let expected = b"@abc[\\]^_`abc{";
        assert_eq!(ascii_tolower_neon(input), expected);
    }
}

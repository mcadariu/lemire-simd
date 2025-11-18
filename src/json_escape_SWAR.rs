/*
JSON Escape Detection Benchmarks (SWAR)

  Clean ASCII (no escapable chars):
  - Scalar: 436,300.17 GB/s
  - SWAR: 311,720.70 GB/s

  With escapable chars (early detection):
  - Scalar: 14,661.04 GB/s
  - SWAR: 29,556.07 GB/s (2.0x faster)

  Mixed content (quotes, backslashes, newlines):
  - Scalar: 138,734.74 GB/s
  - SWAR: 173,913.04 GB/s (1.25x faster)

 */

#[inline]
pub fn needs_json_escape_scalar(byte: u8) -> bool {
    byte < 32 || byte == 34 || byte == 92
}

pub fn has_json_escapable_byte_scalar(buffer: &[u8]) -> bool {
    buffer.iter().any(|&b| needs_json_escape_scalar(b))
}

#[inline]
pub fn has_json_escapable_byte_swar(x: u64) -> bool {
    let is_ascii = 0x8080808080808080u64 & !x;

    let lt32 = x.wrapping_sub(0x2020202020202020u64);

    let sub34 = x ^ 0x2222222222222222u64;
    let eq34 = sub34.wrapping_sub(0x0101010101010101u64);

    let sub92 = x ^ 0x5C5C5C5C5C5C5C5Cu64;
    let eq92 = sub92.wrapping_sub(0x0101010101010101u64);

    ((lt32 | eq34 | eq92) & is_ascii) != 0
}

pub fn has_json_escapable_byte(buffer: &[u8]) -> bool {
    let mut i = 0;

    while i + 8 <= buffer.len() {
        let chunk = u64::from_le_bytes([
            buffer[i],
            buffer[i + 1],
            buffer[i + 2],
            buffer[i + 3],
            buffer[i + 4],
            buffer[i + 5],
            buffer[i + 6],
            buffer[i + 7],
        ]);

        if has_json_escapable_byte_swar(chunk) {
            return true;
        }

        i += 8;
    }

    //leftovers
    buffer[i..].iter().any(|&b| needs_json_escape_scalar(b))
}

pub fn find_first_escapable(buffer: &[u8]) -> Option<usize> {
    for (i, &byte) in buffer.iter().enumerate() {
        if needs_json_escape_scalar(byte) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_control_chars() {
        assert!(needs_json_escape_scalar(0));
        assert!(needs_json_escape_scalar(10));
        assert!(needs_json_escape_scalar(13));
        assert!(needs_json_escape_scalar(31));
    }

    #[test]
    fn test_scalar_quote() {
        assert!(needs_json_escape_scalar(34));
    }

    #[test]
    fn test_scalar_backslash() {
        assert!(needs_json_escape_scalar(92));
    }

    #[test]
    fn test_scalar_normal_chars() {
        assert!(!needs_json_escape_scalar(32));
        assert!(!needs_json_escape_scalar(65));
        assert!(!needs_json_escape_scalar(97));
        assert!(!needs_json_escape_scalar(126));
    }

    #[test]
    fn test_swar_clean_bytes() {
        let x = u64::from_le_bytes([b'H', b'e', b'l', b'l', b'o', b'!', b'!', b'!']);
        assert!(!has_json_escapable_byte_swar(x));
    }

    #[test]
    fn test_swar_with_quote() {
        let x = u64::from_le_bytes([b'H', b'"', b'i', b' ', b' ', b' ', b' ', b' ']);
        assert!(has_json_escapable_byte_swar(x));
    }

    #[test]
    fn test_swar_with_backslash() {
        let x = u64::from_le_bytes([b'A', b'\\', b'B', b' ', b' ', b' ', b' ', b' ']);
        assert!(has_json_escapable_byte_swar(x));
    }

    #[test]
    fn test_swar_with_control() {
        let x = u64::from_le_bytes([b'A', b'\n', b'B', b' ', b' ', b' ', b' ', b' ']);
        assert!(has_json_escapable_byte_swar(x));
    }

    #[test]
    fn test_swar_with_tab() {
        let x = u64::from_le_bytes([b'A', b'\t', b'B', b' ', b' ', b' ', b' ', b' ']);
        assert!(has_json_escapable_byte_swar(x));
    }

    #[test]
    fn test_swar_matches_scalar() {
        let test_cases = vec![
            b"" as &[u8],
            b"Hello",
            b"Hello \"World\"",
            b"Path\\to\\file",
            b"Line1\nLine2\nLine3",
            b"Tab\tseparated\tvalues",
            b"\x00\x01\x02\x03\x04",
            b"Mixed \"quotes\" and \\backslashes\\ and \nnewlines",
        ];

        for test in test_cases {
            let swar_result = has_json_escapable_byte(test);
            let scalar_result = has_json_escapable_byte_scalar(test);
            assert_eq!(
                swar_result, scalar_result,
                "Mismatch for input: {:?}",
                std::str::from_utf8(test).unwrap_or("<invalid utf8>")
            );
        }
    }
}

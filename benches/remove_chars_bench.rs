use std::time::Instant;
use scratchpad::remove_chars_from_strings::{remove_byte_neon, remove_chars_from_strings_scalar};

fn bench_with_timing(name: &str, f: impl Fn() -> usize, iterations: usize, input_size: usize) -> f64 {
    // Warmup
    for _ in 0..10 {
        std::hint::black_box(f());
    }

    let start = Instant::now();
    let mut total_bytes = 0;

    for _ in 0..iterations {
        let result = f();
        total_bytes += input_size;
        std::hint::black_box(result);
    }

    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    let throughput_gb_s = (total_bytes as f64 / elapsed_secs) / 1_000_000_000.0;

    println!(
        "{:30} {:.2} ms total, {:.2} GB/s throughput",
        format!("{}:", name),
        elapsed_secs * 1000.0,
        throughput_gb_s
    );

    throughput_gb_s
}

fn main() {
    println!("Remove Byte from Buffer Benchmarks (ARM NEON)\n");
    println!("Comparing scalar vs NEON (16 bytes/iter)\n");

    let iterations = 1_000;

    // Test 1: Remove space character (moderate frequency)
    println!("=== Test 1: Remove spaces (moderate frequency) ===");
    let space_input: Vec<u8> = b"The quick brown fox jumps over the lazy dog "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_space = bench_with_timing(
        "Scalar",
        || {
            let mut data = space_input.clone();
            remove_chars_from_strings_scalar(&mut data, b' ')
        },
        iterations,
        space_input.len(),
    );

    let neon_space = bench_with_timing(
        "NEON (16 bytes/iter)",
        || unsafe {
            let mut data = space_input.clone();
            remove_byte_neon(&mut data, b' ')
        },
        iterations,
        space_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_space / scalar_space);

    // Test 2: Remove character that doesn't exist (best case)
    println!("=== Test 2: Remove non-existent character (best case) ===");
    let no_match_input: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz0123456789"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_no_match = bench_with_timing(
        "Scalar",
        || {
            let mut data = no_match_input.clone();
            remove_chars_from_strings_scalar(&mut data, b'X')
        },
        iterations,
        no_match_input.len(),
    );

    let neon_no_match = bench_with_timing(
        "NEON (16 bytes/iter)",
        || unsafe {
            let mut data = no_match_input.clone();
            remove_byte_neon(&mut data, b'X')
        },
        iterations,
        no_match_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_no_match / scalar_no_match);

    // Test 3: Remove common character (high frequency)
    println!("=== Test 3: Remove 'a' (high frequency) ===");
    let common_char_input: Vec<u8> = b"bananarama llama drama "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_common = bench_with_timing(
        "Scalar",
        || {
            let mut data = common_char_input.clone();
            remove_chars_from_strings_scalar(&mut data, b'a')
        },
        iterations,
        common_char_input.len(),
    );

    let neon_common = bench_with_timing(
        "NEON (16 bytes/iter)",
        || unsafe {
            let mut data = common_char_input.clone();
            remove_byte_neon(&mut data, b'a')
        },
        iterations,
        common_char_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_common / scalar_common);

    // Test 4: Remove newlines (realistic use case)
    println!("=== Test 4: Remove newlines (realistic) ===");
    let newline_input: Vec<u8> = b"line1\nline2\nline3\nline4\nline5\n"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_newline = bench_with_timing(
        "Scalar",
        || {
            let mut data = newline_input.clone();
            remove_chars_from_strings_scalar(&mut data, b'\n')
        },
        iterations,
        newline_input.len(),
    );

    let neon_newline = bench_with_timing(
        "NEON (16 bytes/iter)",
        || unsafe {
            let mut data = newline_input.clone();
            remove_byte_neon(&mut data, b'\n')
        },
        iterations,
        newline_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_newline / scalar_newline);

    // Test 5: Remove every other byte (worst case)
    println!("=== Test 5: Remove alternating pattern (worst case) ===");
    let alternating_input: Vec<u8> = b"ababababababababababababababababab"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_alternating = bench_with_timing(
        "Scalar",
        || {
            let mut data = alternating_input.clone();
            remove_chars_from_strings_scalar(&mut data, b'a')
        },
        iterations,
        alternating_input.len(),
    );

    let neon_alternating = bench_with_timing(
        "NEON (16 bytes/iter)",
        || unsafe {
            let mut data = alternating_input.clone();
            remove_byte_neon(&mut data, b'a')
        },
        iterations,
        alternating_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_alternating / scalar_alternating);

    // Summary
    println!("=== Summary ===");
    let avg_speedup = (neon_space / scalar_space
        + neon_no_match / scalar_no_match
        + neon_common / scalar_common
        + neon_newline / scalar_newline
        + neon_alternating / scalar_alternating) / 5.0;

    println!("  NEON average speedup: {:.2}x", avg_speedup);
}

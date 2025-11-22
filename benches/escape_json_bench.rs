use std::time::Instant;
use scratchpad::escape_strings::{escape_json_neon, escape_json_scalar};

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
    println!("JSON String Escaping Benchmarks (ARM NEON)\n");
    println!("Comparing scalar vs NEON (8 bytes/iter)\n");

    let iterations = 1_000;

    // Test 1: No escaping needed (best case)
    println!("=== Test 1: No escaping needed (best case) ===");
    let no_escape_input: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz0123456789 "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_no_escape = bench_with_timing(
        "Scalar",
        || {
            let mut output = vec![0u8; no_escape_input.len() * 2];
            escape_json_scalar(&no_escape_input, &mut output)
        },
        iterations,
        no_escape_input.len(),
    );

    let neon_no_escape = bench_with_timing(
        "NEON (8 bytes/iter)",
        || unsafe {
            let mut output = vec![0u8; no_escape_input.len() * 2];
            escape_json_neon(&no_escape_input, &mut output)
        },
        iterations,
        no_escape_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_no_escape / scalar_no_escape);

    // Test 2: Heavy escaping (worst case - many quotes and backslashes)
    println!("=== Test 2: Heavy escaping (worst case) ===");
    let heavy_escape_input: Vec<u8> = b"\"test\\path\" \"another\\one\" "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_heavy = bench_with_timing(
        "Scalar",
        || {
            let mut output = vec![0u8; heavy_escape_input.len() * 2];
            escape_json_scalar(&heavy_escape_input, &mut output)
        },
        iterations,
        heavy_escape_input.len(),
    );

    let neon_heavy = bench_with_timing(
        "NEON (8 bytes/iter)",
        || unsafe {
            let mut output = vec![0u8; heavy_escape_input.len() * 2];
            escape_json_neon(&heavy_escape_input, &mut output)
        },
        iterations,
        heavy_escape_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_heavy / scalar_heavy);

    // Test 3: Realistic JSON text
    println!("=== Test 3: Realistic JSON text ===");
    let realistic_input: Vec<u8> = b"{\"name\":\"John\",\"path\":\"C:\\\\Users\\\\John\",\"age\":30}"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_realistic = bench_with_timing(
        "Scalar",
        || {
            let mut output = vec![0u8; realistic_input.len() * 2];
            escape_json_scalar(&realistic_input, &mut output)
        },
        iterations,
        realistic_input.len(),
    );

    let neon_realistic = bench_with_timing(
        "NEON (8 bytes/iter)",
        || unsafe {
            let mut output = vec![0u8; realistic_input.len() * 2];
            escape_json_neon(&realistic_input, &mut output)
        },
        iterations,
        realistic_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_realistic / scalar_realistic);

    // Test 4: Only quotes
    println!("=== Test 4: Only quotes ===");
    let quotes_input: Vec<u8> = b"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\" "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_quotes = bench_with_timing(
        "Scalar",
        || {
            let mut output = vec![0u8; quotes_input.len() * 2];
            escape_json_scalar(&quotes_input, &mut output)
        },
        iterations,
        quotes_input.len(),
    );

    let neon_quotes = bench_with_timing(
        "NEON (8 bytes/iter)",
        || unsafe {
            let mut output = vec![0u8; quotes_input.len() * 2];
            escape_json_neon(&quotes_input, &mut output)
        },
        iterations,
        quotes_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_quotes / scalar_quotes);

    // Test 5: Only backslashes
    println!("=== Test 5: Only backslashes ===");
    let backslash_input: Vec<u8> = b"\\\\\\\\\\\\\\\\ "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_backslash = bench_with_timing(
        "Scalar",
        || {
            let mut output = vec![0u8; backslash_input.len() * 2];
            escape_json_scalar(&backslash_input, &mut output)
        },
        iterations,
        backslash_input.len(),
    );

    let neon_backslash = bench_with_timing(
        "NEON (8 bytes/iter)",
        || unsafe {
            let mut output = vec![0u8; backslash_input.len() * 2];
            escape_json_neon(&backslash_input, &mut output)
        },
        iterations,
        backslash_input.len(),
    );

    println!("  NEON speedup: {:.2}x\n", neon_backslash / scalar_backslash);

    // Summary
    println!("=== Summary ===");
    let avg_speedup = (neon_no_escape / scalar_no_escape
        + neon_heavy / scalar_heavy
        + neon_realistic / scalar_realistic
        + neon_quotes / scalar_quotes
        + neon_backslash / scalar_backslash) / 5.0;

    println!("  NEON average speedup: {:.2}x", avg_speedup);
}

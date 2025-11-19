use std::time::Instant;
use scratchpad::ascii_tolower_neon::{ascii_tolower_neon, ascii_tolower_neon_32, ascii_tolower_neon_64, ascii_tolower_scalar};

fn bench_with_timing(name: &str, f: impl Fn() -> Vec<u8>, iterations: usize, input_size: usize) -> f64 {
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
    println!("ASCII to Lowercase Conversion Benchmarks (ARM NEON)\n");
    println!("Comparing scalar vs NEON (16B) vs NEON (32B) vs NEON (64B)\n");

    let iterations = 1_000;

    // Test 1: All uppercase letters
    println!("=== Test 1: All uppercase letters (worst case) ===");
    let uppercase_input: Vec<u8> = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_upper = bench_with_timing(
        "Scalar",
        || ascii_tolower_scalar(&uppercase_input),
        iterations,
        uppercase_input.len(),
    );

    let neon_upper = bench_with_timing(
        "NEON (16 bytes/iter)",
        || ascii_tolower_neon(&uppercase_input),
        iterations,
        uppercase_input.len(),
    );

    let neon32_upper = bench_with_timing(
        "NEON (32 bytes/iter)",
        || ascii_tolower_neon_32(&uppercase_input),
        iterations,
        uppercase_input.len(),
    );

    let neon64_upper = bench_with_timing(
        "NEON (64 bytes/iter)",
        || ascii_tolower_neon_64(&uppercase_input),
        iterations,
        uppercase_input.len(),
    );

    println!("  NEON-16 speedup: {:.2}x", neon_upper / scalar_upper);
    println!("  NEON-32 speedup: {:.2}x", neon32_upper / scalar_upper);
    println!("  NEON-64 speedup: {:.2}x\n", neon64_upper / scalar_upper);

    // Test 2: All lowercase letters (best case - no conversions needed)
    println!("=== Test 2: All lowercase letters (best case) ===");
    let lowercase_input: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_lower = bench_with_timing(
        "Scalar",
        || ascii_tolower_scalar(&lowercase_input),
        iterations,
        lowercase_input.len(),
    );

    let neon_lower = bench_with_timing(
        "NEON (16 bytes/iter)",
        || ascii_tolower_neon(&lowercase_input),
        iterations,
        lowercase_input.len(),
    );

    let neon32_lower = bench_with_timing(
        "NEON (32 bytes/iter)",
        || ascii_tolower_neon_32(&lowercase_input),
        iterations,
        lowercase_input.len(),
    );

    let neon64_lower = bench_with_timing(
        "NEON (64 bytes/iter)",
        || ascii_tolower_neon_64(&lowercase_input),
        iterations,
        lowercase_input.len(),
    );

    println!("  NEON-16 speedup: {:.2}x", neon_lower / scalar_lower);
    println!("  NEON-32 speedup: {:.2}x", neon32_lower / scalar_lower);
    println!("  NEON-64 speedup: {:.2}x\n", neon64_lower / scalar_lower);

    // Test 3: Mixed case text
    println!("=== Test 3: Mixed case text (realistic) ===");
    let mixed_input: Vec<u8> = b"The Quick BROWN Fox Jumps Over The Lazy DOG 0123456789 "
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_mixed = bench_with_timing(
        "Scalar",
        || ascii_tolower_scalar(&mixed_input),
        iterations,
        mixed_input.len(),
    );

    let neon_mixed = bench_with_timing(
        "NEON (16 bytes/iter)",
        || ascii_tolower_neon(&mixed_input),
        iterations,
        mixed_input.len(),
    );

    let neon32_mixed = bench_with_timing(
        "NEON (32 bytes/iter)",
        || ascii_tolower_neon_32(&mixed_input),
        iterations,
        mixed_input.len(),
    );

    let neon64_mixed = bench_with_timing(
        "NEON (64 bytes/iter)",
        || ascii_tolower_neon_64(&mixed_input),
        iterations,
        mixed_input.len(),
    );

    println!("  NEON-16 speedup: {:.2}x", neon_mixed / scalar_mixed);
    println!("  NEON-32 speedup: {:.2}x", neon32_mixed / scalar_mixed);
    println!("  NEON-64 speedup: {:.2}x\n", neon64_mixed / scalar_mixed);

    // Test 4: Mixed with lots of non-alphabetic chars
    println!("=== Test 4: Numbers and symbols (no conversion) ===");
    let symbols_input: Vec<u8> = b"0123456789 !@#$%^&*()_+-=[]{}|;:',.<>?/~`"
        .iter()
        .cycle()
        .take(1_000_000)
        .copied()
        .collect();

    let scalar_symbols = bench_with_timing(
        "Scalar",
        || ascii_tolower_scalar(&symbols_input),
        iterations,
        symbols_input.len(),
    );

    let neon_symbols = bench_with_timing(
        "NEON (16 bytes/iter)",
        || ascii_tolower_neon(&symbols_input),
        iterations,
        symbols_input.len(),
    );

    let neon32_symbols = bench_with_timing(
        "NEON (32 bytes/iter)",
        || ascii_tolower_neon_32(&symbols_input),
        iterations,
        symbols_input.len(),
    );

    let neon64_symbols = bench_with_timing(
        "NEON (64 bytes/iter)",
        || ascii_tolower_neon_64(&symbols_input),
        iterations,
        symbols_input.len(),
    );

    println!("  NEON-16 speedup: {:.2}x", neon_symbols / scalar_symbols);
    println!("  NEON-32 speedup: {:.2}x", neon32_symbols / scalar_symbols);
    println!("  NEON-64 speedup: {:.2}x\n", neon64_symbols / scalar_symbols);

    // Summary
    println!("=== Summary ===");
    let avg_neon16 = (neon_upper / scalar_upper + neon_lower / scalar_lower + neon_mixed / scalar_mixed + neon_symbols / scalar_symbols) / 4.0;
    let avg_neon32 = (neon32_upper / scalar_upper + neon32_lower / scalar_lower + neon32_mixed / scalar_mixed + neon32_symbols / scalar_symbols) / 4.0;
    let avg_neon64 = (neon64_upper / scalar_upper + neon64_lower / scalar_lower + neon64_mixed / scalar_mixed + neon64_symbols / scalar_symbols) / 4.0;

    println!("  NEON-16 average speedup: {:.2}x", avg_neon16);
    println!("  NEON-32 average speedup: {:.2}x", avg_neon32);
    println!("  NEON-64 average speedup: {:.2}x", avg_neon64);
}

use std::time::Instant;
use scratchpad::json_escape_SWAR::{has_json_escapable_byte, has_json_escapable_byte_scalar};

fn bench_with_timing(name: &str, f: impl Fn() -> bool, iterations: usize, input_size: usize) -> f64 {
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
    println!("JSON Escape Detection Benchmarks (SWAR)\n");

    println!("Clean ASCII (no escapable chars)");
    let clean_input: Vec<u8> = (32..127).cycle().take(1_000_000).collect();
    let iterations = 1_000;

    let scalar_clean = bench_with_timing(
        "Scalar (clean, 1 MB)",
        || has_json_escapable_byte_scalar(&clean_input),
        iterations,
        clean_input.len(),
    );

    let swar_clean = bench_with_timing(
        "SWAR (clean, 1 MB)",
        || has_json_escapable_byte(&clean_input),
        iterations,
        clean_input.len(),
    );

    println!();

    println!("With escapable chars (early detection)");
    let mut early_escape = vec![65u8; 1_000_000];
    early_escape[100] = b'"';

    let scalar_early = bench_with_timing(
        "Scalar (early escape, 1 MB)",
        || has_json_escapable_byte_scalar(&early_escape),
        iterations,
        early_escape.len(),
    );

    let swar_early = bench_with_timing(
        "SWAR (early escape, 1 MB)",
        || has_json_escapable_byte(&early_escape),
        iterations,
        early_escape.len(),
    );

    println!();

    println!("Mixed content (quotes, backslashes, newlines)");
    let mut mixed_input = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        let byte = match i % 100 {
            10 => b'"',
            25 => b'\\',
            50 => b'\n',
            75 => b'\t',
            _ => (65 + (i % 26)) as u8,
        };
        mixed_input.push(byte);
    }

    let scalar_mixed = bench_with_timing(
        "Scalar (mixed, 1 MB)",
        || has_json_escapable_byte_scalar(&mixed_input),
        iterations,
        mixed_input.len(),
    );

    let swar_mixed = bench_with_timing(
        "SWAR (mixed, 1 MB)",
        || has_json_escapable_byte(&mixed_input),
        iterations,
        mixed_input.len(),
    );

    println!();
}

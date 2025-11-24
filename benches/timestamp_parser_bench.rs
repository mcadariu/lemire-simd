use std::time::Instant;
use scratchpad::timestamp_parser_neon::{validate_timestamp_neon, validate_timestamp_scalar};

fn bench_with_timing(name: &str, f: impl Fn() -> bool, iterations: usize) -> f64 {
    for _ in 0..10 {
        std::hint::black_box(f());
    }

    let start = Instant::now();

    for _ in 0..iterations {
        let result = f();
        std::hint::black_box(result);
    }

    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    let ops_per_sec = iterations as f64 / elapsed_secs;

    println!(
        "{:30} {:.2} ms total, {:.2} M validations/sec",
        format!("{}:", name),
        elapsed_secs * 1000.0,
        ops_per_sec / 1_000_000.0
    );

    ops_per_sec
}

fn main() {
    println!("Timestamp Validation Benchmarks (ARM NEON)\n");
    println!("Comparing scalar vs NEON for YYYYMMDDHHMMSS format\n");

    let iterations = 10_000_000;

    println!("=== Test 1: Valid timestamp ===");
    let valid = b"20241124153045XX";

    let scalar_valid = bench_with_timing(
        "Scalar",
        || validate_timestamp_scalar(valid),
        iterations,
    );

    let neon_valid = bench_with_timing(
        "NEON",
        || unsafe { validate_timestamp_neon(valid) },
        iterations,
    );

    println!("  NEON speedup: {:.2}x\n", neon_valid / scalar_valid);

    println!("=== Test 2: Invalid month (13) ===");
    let invalid_month = b"20241324153045XX";

    let scalar_invalid_month = bench_with_timing(
        "Scalar",
        || validate_timestamp_scalar(invalid_month),
        iterations,
    );

    let neon_invalid_month = bench_with_timing(
        "NEON",
        || unsafe { validate_timestamp_neon(invalid_month) },
        iterations,
    );

    println!("  NEON speedup: {:.2}x\n", neon_invalid_month / scalar_invalid_month);

    println!("=== Test 3: Invalid hour (24) ===");
    let invalid_hour = b"20241124243045XX";

    let scalar_invalid_hour = bench_with_timing(
        "Scalar",
        || validate_timestamp_scalar(invalid_hour),
        iterations,
    );

    let neon_invalid_hour = bench_with_timing(
        "NEON",
        || unsafe { validate_timestamp_neon(invalid_hour) },
        iterations,
    );

    println!("  NEON speedup: {:.2}x\n", neon_invalid_hour / scalar_invalid_hour);

    println!("=== Test 4: Invalid minute (60) ===");
    let invalid_minute = b"20241124156045XX";

    let scalar_invalid_minute = bench_with_timing(
        "Scalar",
        || validate_timestamp_scalar(invalid_minute),
        iterations,
    );

    let neon_invalid_minute = bench_with_timing(
        "NEON",
        || unsafe { validate_timestamp_neon(invalid_minute) },
        iterations,
    );

    println!("  NEON speedup: {:.2}x\n", neon_invalid_minute / scalar_invalid_minute);

    println!("=== Test 5: Edge case - Dec 31st 23:59:59 ===");
    let edge_case = b"20241231235959XX";

    let scalar_edge = bench_with_timing(
        "Scalar",
        || validate_timestamp_scalar(edge_case),
        iterations,
    );

    let neon_edge = bench_with_timing(
        "NEON",
        || unsafe { validate_timestamp_neon(edge_case) },
        iterations,
    );

    println!("  NEON speedup: {:.2}x\n", neon_edge / scalar_edge);

    println!("=== Summary ===");
    let avg_speedup = (neon_valid / scalar_valid
        + neon_invalid_month / scalar_invalid_month
        + neon_invalid_hour / scalar_invalid_hour
        + neon_invalid_minute / scalar_invalid_minute
        + neon_edge / scalar_edge) / 5.0;

    println!("  NEON average speedup: {:.2}x", avg_speedup);

    println!("\n=== BATCH PROCESSING TEST ===");
    println!("Processing arrays of timestamps (amortizes SIMD overhead)\n");

    let batch_sizes = [100, 1000, 10000];

    for &batch_size in &batch_sizes {
        println!("--- Batch size: {} timestamps ---", batch_size);

        let mut batch: Vec<&[u8]> = Vec::with_capacity(batch_size);
        for i in 0..batch_size {
            if i % 3 == 0 {
                batch.push(b"20241124153045XX");
            } else if i % 3 == 1 {
                batch.push(b"20241231235959XX");
            } else {
                batch.push(b"20240101000000XX");
            }
        }

        let iterations_batch = 100_000;

        let scalar_batch = bench_with_timing(
            "Scalar",
            || {
                let mut valid_count = 0;
                for &ts in &batch {
                    if validate_timestamp_scalar(ts) {
                        valid_count += 1;
                    }
                }
                valid_count == batch_size
            },
            iterations_batch,
        );

        let neon_batch = bench_with_timing(
            "NEON",
            || unsafe {
                let mut valid_count = 0;
                for &ts in &batch {
                    if validate_timestamp_neon(ts) {
                        valid_count += 1;
                    }
                }
                valid_count == batch_size
            },
            iterations_batch,
        );

        println!("  NEON speedup: {:.2}x\n", neon_batch / scalar_batch);
    }
}

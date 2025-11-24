use std::time::Instant;
use scratchpad::ipv4_parser_neon::{parse_ipv4_neon, parse_ipv4_scalar};

fn bench_with_timing(name: &str, f: impl Fn() -> Option<[u8; 4]>, iterations: usize) -> f64 {
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
        "{:30} {:.2} ms total, {:.2} M parses/sec",
        format!("{}:", name),
        elapsed_secs * 1000.0,
        ops_per_sec / 1_000_000.0
    );

    ops_per_sec
}

fn main() {
    println!("IPv4 Parser Benchmarks (ARM NEON)\n");
    println!("Format: XXX.XXX.XXX.XXX (fixed-width with leading zeros)\n");

    let iterations = 10_000_000;

    println!("=== Single IP Parsing ===");
    let valid_ip = b"192.168.001.255X";

    let scalar_single = bench_with_timing(
        "Scalar",
        || parse_ipv4_scalar(valid_ip),
        iterations,
    );

    let neon_single = bench_with_timing(
        "NEON",
        || unsafe { parse_ipv4_neon(valid_ip) },
        iterations,
    );

    println!("  NEON speedup: {:.2}x\n", neon_single / scalar_single);

    println!("=== Batch Processing ===");
    let batch_sizes = [100, 1000, 10000];

    for &batch_size in &batch_sizes {
        println!("--- Batch size: {} IPs ---", batch_size);

        let mut batch: Vec<&[u8]> = Vec::with_capacity(batch_size);
        for i in 0..batch_size {
            match i % 4 {
                0 => batch.push(b"192.168.001.001X"),
                1 => batch.push(b"010.000.000.001X"),
                2 => batch.push(b"172.016.000.001X"),
                _ => batch.push(b"255.255.255.255X"),
            }
        }

        let iterations_batch = 100_000;

        let scalar_batch = bench_with_timing(
            "Scalar",
            || {
                for &ip in &batch {
                    std::hint::black_box(parse_ipv4_scalar(ip));
                }
                Some([0, 0, 0, 0])
            },
            iterations_batch,
        );

        let neon_batch = bench_with_timing(
            "NEON",
            || unsafe {
                for &ip in &batch {
                    std::hint::black_box(parse_ipv4_neon(ip));
                }
                Some([0, 0, 0, 0])
            },
            iterations_batch,
        );

        println!("  NEON speedup: {:.2}x\n", neon_batch / scalar_batch);
    }
}

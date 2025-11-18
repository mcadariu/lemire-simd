/*
Line Feed Insertion every K bytes

Large input (1 MB, K=64)
Scalar (large):                57.98 ms total, 17.52 GB/s throughput
NEON (large):                  23.36 ms total, 43.49 GB/s throughput

Very large input (10 MB, K=64)
Scalar (very large):           38.16 ms total, 26.61 GB/s throughput
NEON (very large):             23.79 ms total, 42.69 GB/s throughput

Different K values (1 MB input)
Scalar (K=32):                 67.30 ms total, 7.66 GB/s throughput
NEON (K=32):                   11.57 ms total, 44.58 GB/s throughput

Scalar (K=64):                 18.47 ms total, 27.49 GB/s throughput
NEON (K=64):                   11.62 ms total, 43.70 GB/s throughput

Scalar (K=72):                 17.85 ms total, 28.40 GB/s throughput
NEON (K=72):                   22.09 ms total, 22.95 GB/s throughput

Scalar (K=128):                15.09 ms total, 33.38 GB/s throughput
NEON (K=128):                  10.49 ms total, 48.03 GB/s throughput
 */

use std::arch::aarch64::*;

pub static SHUFFLE_MASKS_NEON: [[u8; 16]; 16] = [
    [255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 255, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 255, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 255, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 255, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 255, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 255, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 255, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 255, 8, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 255, 9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 255, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 255, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 255, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 255],
];

#[target_feature(enable = "neon")]
pub unsafe fn insert_line_feed32_neon_impl(input: &[u8; 32], n: usize) -> [u8; 33] {
    let mut output = [0u8; 33];

    let lower = vld1q_u8(input.as_ptr());
    let upper = vld1q_u8(input.as_ptr().add(16));

    let line_feed_vector = vdupq_n_u8(b'\n');
    let identity = vcombine_u8(
        vcreate_u8(0x0706050403020100u64),
        vcreate_u8(0x0F0E0D0C0B0A0908u64),
    );

    if n == 32 {
        vst1q_u8(output.as_mut_ptr(), lower);
        vst1q_u8(output.as_mut_ptr().add(16), upper);
        output[32] = b'\n';

    } else if n >= 16 {
        let maskhi = vld1q_u8(SHUFFLE_MASKS_NEON[n - 16].as_ptr());

        let lf_pos_lo = vceqq_u8(identity, vdupq_n_u8(255));
        let shuffled_lo = vqtbl1q_u8(lower, identity);
        let result_lo = vbslq_u8(lf_pos_lo, line_feed_vector, shuffled_lo);

        let lf_pos_hi = vceqq_u8(maskhi, vdupq_n_u8(255));
        let shuffled_hi = vqtbl1q_u8(upper, maskhi);
        let result_hi = vbslq_u8(lf_pos_hi, line_feed_vector, shuffled_hi);

        vst1q_u8(output.as_mut_ptr(), result_lo);
        vst1q_u8(output.as_mut_ptr().add(16), result_hi);

        output[32] = input[31];

    } else {
        let shifted_upper = vextq_u8(lower, upper, 15);

        let masklo = vld1q_u8(SHUFFLE_MASKS_NEON[n].as_ptr());
        let lf_pos_lo = vceqq_u8(masklo, vdupq_n_u8(255));
        let shuffled_lo = vqtbl1q_u8(lower, masklo);
        let result_lo = vbslq_u8(lf_pos_lo, line_feed_vector, shuffled_lo);

        let lf_pos_hi = vceqq_u8(identity, vdupq_n_u8(255));
        let shuffled_hi = vqtbl1q_u8(shifted_upper, identity);
        let result_hi = vbslq_u8(lf_pos_hi, line_feed_vector, shuffled_hi);

        vst1q_u8(output.as_mut_ptr(), result_lo);
        vst1q_u8(output.as_mut_ptr().add(16), result_hi);

        output[32] = input[31];
    }

    output
}

pub fn insert_line_feed_scalar(buffer: &[u8], k: usize) -> Vec<u8> {
    if k == 0 {
        return buffer.to_vec();
    }

    let num_line_feeds = buffer.len() / k;
    let output_len = buffer.len() + num_line_feeds;
    let mut output = Vec::with_capacity(output_len);

    let mut input_pos = 0;

    while input_pos + k <= buffer.len() {
        output.extend_from_slice(&buffer[input_pos..input_pos + k]);
        output.push(b'\n');
        input_pos += k;
    }

    output.extend_from_slice(&buffer[input_pos..]);

    output
}

pub fn insert_line_feed_neon(buffer: &[u8], k: usize) -> Vec<u8> {
    if k == 0 {
        return buffer.to_vec();
    }

    let num_line_feeds = buffer.len() / k;
    let output_len = buffer.len() + num_line_feeds;
    let mut output = Vec::with_capacity(output_len);

    let mut input_pos = 0;

    unsafe {
        let output_ptr: *mut u8 = output.as_mut_ptr();
        let mut output_pos = 0;

        while input_pos + k <= buffer.len() {
            if k <= 32 {
                let input_ptr = buffer.as_ptr().add(input_pos);

                let lower = vld1q_u8(input_ptr);
                let upper = if input_pos + 16 < buffer.len() {
                    vld1q_u8(input_ptr.add(16))
                } else {
                    vdupq_n_u8(0)
                };

                let line_feed_vector = vdupq_n_u8(b'\n');
                let identity = vcombine_u8(
                    vcreate_u8(0x0706050403020100u64),
                    vcreate_u8(0x0F0E0D0C0B0A0908u64),
                );

                if k == 32 {
                    vst1q_u8(output_ptr.add(output_pos), lower);
                    vst1q_u8(output_ptr.add(output_pos + 16), upper);
                    *output_ptr.add(output_pos + 32) = b'\n';
                    output_pos += 33;
                } else if k >= 16 {
                    let maskhi = vld1q_u8(SHUFFLE_MASKS_NEON[k - 16].as_ptr());

                    let lf_pos_lo = vceqq_u8(identity, vdupq_n_u8(255));
                    let shuffled_lo = vqtbl1q_u8(lower, identity);
                    let result_lo = vbslq_u8(lf_pos_lo, line_feed_vector, shuffled_lo);

                    let lf_pos_hi = vceqq_u8(maskhi, vdupq_n_u8(255));
                    let shuffled_hi = vqtbl1q_u8(upper, maskhi);
                    let result_hi = vbslq_u8(lf_pos_hi, line_feed_vector, shuffled_hi);

                    vst1q_u8(output_ptr.add(output_pos), result_lo);
                    vst1q_u8(output_ptr.add(output_pos + 16), result_hi);
                    output_pos += k + 1;
                } else {
                    let shifted_upper = vextq_u8(lower, upper, 15);

                    let masklo = vld1q_u8(SHUFFLE_MASKS_NEON[k].as_ptr());
                    let lf_pos_lo = vceqq_u8(masklo, vdupq_n_u8(255));
                    let shuffled_lo = vqtbl1q_u8(lower, masklo);
                    let result_lo = vbslq_u8(lf_pos_lo, line_feed_vector, shuffled_lo);

                    let lf_pos_hi = vceqq_u8(identity, vdupq_n_u8(255));
                    let shuffled_hi = vqtbl1q_u8(shifted_upper, identity);
                    let result_hi = vbslq_u8(lf_pos_hi, line_feed_vector, shuffled_hi);

                    vst1q_u8(output_ptr.add(output_pos), result_lo);
                    vst1q_u8(output_ptr.add(output_pos + 16), result_hi);
                    output_pos += k + 1;
                }

                input_pos += k;
            } else {
                let mut remaining = k;

                while remaining >= 32 {
                    let input_ptr = buffer.as_ptr().add(input_pos);

                    let lower = vld1q_u8(input_ptr);
                    let upper = vld1q_u8(input_ptr.add(16));

                    vst1q_u8(output_ptr.add(output_pos), lower);
                    vst1q_u8(output_ptr.add(output_pos + 16), upper);

                    output_pos += 32;
                    input_pos += 32;
                    remaining -= 32;
                }

                if remaining > 0 {
                    std::ptr::copy_nonoverlapping(
                        buffer.as_ptr().add(input_pos),
                        output_ptr.add(output_pos),
                        remaining
                    );
                    output_pos += remaining;
                    input_pos += remaining;
                }

                *output_ptr.add(output_pos) = b'\n';
                output_pos += 1;
            }
        }

        output.set_len(output_pos);
    }

    output.extend_from_slice(&buffer[input_pos..]);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_basic() {
        let input = b"ABCDEFGHIJ";
        let result = insert_line_feed_scalar(input, 3);
        assert_eq!(result, b"ABC\nDEF\nGHI\nJ");
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_neon_matches_scalar_small() {
        let input = b"ABCDEFGHIJ";
        let scalar = insert_line_feed_scalar(input, 3);
        let neon = insert_line_feed_neon(input, 3);
        assert_eq!(scalar, neon, "NEON and scalar results should match for small input");
    }
}

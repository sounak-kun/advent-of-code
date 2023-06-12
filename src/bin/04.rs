use std::io::{self, prelude::*};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

const THREADS: usize = 12;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let input = input.trim();

    // Part 1
    let secret = AtomicUsize::new(0);
    let found = AtomicBool::new(false);
    thread::scope(|s| {
        for _ in 0..THREADS {
            s.spawn(|| {
                while !found.load(Ordering::Relaxed) {
                    let secret = secret.fetch_add(1, Ordering::Relaxed);
                    let hash = md5((input.to_string() + &secret.to_string()).as_bytes());
                    // 128 = length of hash in bits,
                    // 4 * 5 = 5 leading hex zeroes
                    if hash >> (128 - 4 * 5) == 0 {
                        println!("Part 1: {}", secret);
                        found.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            });
        }
    });

    // Part 2
    let secret = AtomicUsize::new(0);
    let found = AtomicBool::new(false);
    thread::scope(|s| {
        for _ in 0..THREADS {
            s.spawn(|| {
                while !found.load(Ordering::Relaxed) {
                    let secret = secret.fetch_add(1, Ordering::Relaxed);
                    let hash = md5((input.to_string() + &secret.to_string()).as_bytes());
                    // 128 = length of hash in bits,
                    // 4 * 6 = 6 leading hex zeroes
                    if hash >> (128 - 4 * 6) == 0 {
                        println!("Part 2: {}", secret);
                        found.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            });
        }
    });
}

fn md5(input: &[u8]) -> u128 {
    // Initialize variables
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    // Pre-processing
    let mut input = input.to_vec();
    let input_len = input.len() * 8;
    input.push(0x80);
    // 64 bytes = 512 bits
    // Subtract 8 bytes for the length
    input.extend((0..(64 - (input.len() % 64) - 8)).map(|_| 0x00));
    input.extend(input_len.to_le_bytes());

    // Process the message in successive 512-bit chunks
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    input.chunks_exact(64).for_each(|chunk| {
        // Break chunk into sixteen 32-bit words
        let m: Vec<u32> = chunk
            .chunks_exact(4)
            .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
            .collect();

        // Initialize hash value for this chunk
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        // Main loop
        for i in 0..64 {
            let (mut f, g) = match i {
                00..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                48..=63 => (c ^ (b | !d), (7 * i) % 16),
                _ => unreachable!(),
            };
            f = f
                .wrapping_add(a)
                .wrapping_add(K[i])
                .wrapping_add(m[g])
                .rotate_left(S[i]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f);
        }

        // Add this chunk's hash to result so far
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    });

    // Produce the final hash value
    ((d0 as u128) << 96 | (c0 as u128) << 64 | (b0 as u128) << 32 | (a0 as u128)).to_be()
}

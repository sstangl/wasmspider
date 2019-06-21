//! Rust implementation of bitops-nsieve-bits.

// Original code part of The Great Computer Language Shootout,
// http://shootout.alioth.debian.org, contributed by Ian Osgood.

use std::num::Wrapping;

fn primes(mut is_prime: Vec<Wrapping<u32>>, n: u32) -> i64 {
    let m = 10000 << n;

    for i in 2..m {
        if (is_prime[i >> 5] & Wrapping(1 << (i & 31))) > Wrapping(0) {
            for j in (i+1..m).step_by(i) {
                is_prime[j >> 5] &= Wrapping(!(1 << (j & 31)));
            }
        }
    }

    let mut sum: i64 = 0;
    for v in is_prime.iter() {
        sum += (v.0 as i32) as i64;
    }

    sum
}

fn sieve() -> i64 {
    let mut acc = 0;
    for i in 0..4 {
        let mut is_prime: Vec<Wrapping<u32>> = Vec::new();
        is_prime.resize((10000 << i) + 31 >> 5, Wrapping(0xffffffff));
        acc = primes(is_prime, i);
    }

    acc
}

#[no_mangle]
pub fn bitops_nsieve_bits() -> u32 {
    sieve() as u32
}

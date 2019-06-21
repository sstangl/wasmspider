//! Rust implementation of SunSpider's bitops-3bit-bits-in-byte.

// Original version bears the following copyright:
// Copyright (c) 2004 by Arthur Langereis (arthur_ext@xfinitegames.com).

fn fast3bitlookup(b: u32) -> u32 {
    let mut c: u32;
    let bi3b: u32 = 0xE994;

    c  = 3 & (bi3b >> ((b << 1) & 14));
    c += 3 & (bi3b >> ((b >> 2) & 14));
    c += 3 & (bi3b >> ((b >> 5) & 6));

    c
}

fn timefunc(func: fn(u32) -> u32) -> u32 {
    let mut sum = 0;
    for _ in 0..500 {
        for y in 0..256 {
            sum += func(y);
        }
    }
    sum
}

#[no_mangle]
pub fn bitops_3bit_bits_in_byte() -> u32 {
    timefunc(fast3bitlookup)
}

#[cfg(test)]
mod tests {
    use super::bitops_3bit_bits_in_byte;

    #[test]
    fn test_bitops_3bit_bits_in_byte() {
        assert_eq!(bitops_3bit_bits_in_byte(), 512000);
    }
}

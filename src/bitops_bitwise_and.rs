//! Rust implementation of SunSpider's bitopts-bitwise-and test.

#[no_mangle]
pub fn bitops_bitwise_and() -> u32 {
    // The original SunSpider test uses a global.
    let mut result: Box<u64> = Box::new(4294967296);

    for i in 0..600000 {
         *result = *result & i;
    }

    // Inhibit dead code elimination.
    *result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitops_bitwise_and() {
        assert_eq!(bitops_bitwise_and(), 0);
    }
}

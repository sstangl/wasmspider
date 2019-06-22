//! Defines functions exported by the wasm module.

mod access_binary_trees;
pub use access_binary_trees::*;

// FIXME: Disabled: There seems to be a wasm compilation error, where array
// index randomly fails in-browser with "RuntimeError: unreachable executed".
// mod access_fannkuch;
// pub use access_fannkuch::*;

mod bitops_bitwise_and;
pub use bitops_bitwise_and::*;

mod bitops_3bit_bits_in_byte;
pub use bitops_3bit_bits_in_byte::*;

mod bitops_nsieve_bits;
pub use bitops_nsieve_bits::*;

mod controlflow_recursive;
pub use controlflow_recursive::*;

mod math_partial_sums;
pub use math_partial_sums::*;

mod string_fasta;
pub use string_fasta::*;

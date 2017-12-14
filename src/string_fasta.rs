//! Rust implementation of string-fasta.
// The original implemetation is part of The Great Computer Language Shootout,
// contributed by Ian Osgood.

use std::collections::HashMap;

fn rand(max: i32) -> f64 {
    let mut last = 42;
    let a = 3877;
    let c = 29573;
    let m = 139968;

    last = (last * a + c) % m;
    return (max * last) as f64 / (m as f64);
}

const ALU: &'static str = "\
GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGG\
GAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGA\
CCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAAT\
ACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCA\
GCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGG\
AGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCC\
AGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA"; 

fn make_cumulative(map: &mut HashMap<char, f64>) {
    let mut last: Option<f64> = None;
    for val in map.values_mut() {
        if let Some(ref last_val) = last {
            *val += last_val;
        }
        last = Some(*val);
    }
}

fn make_iub_hashmap() -> HashMap<char, f64> {
    let mut iub = HashMap::new();
    iub.insert('a', 0.27);
    iub.insert('c', 0.12);
    iub.insert('g', 0.12);
    iub.insert('t', 0.27);
    iub.insert('B', 0.02);
    iub.insert('D', 0.02);
    iub.insert('H', 0.02);
    iub.insert('K', 0.02);
    iub.insert('M', 0.02);
    iub.insert('N', 0.02);
    iub.insert('R', 0.02);
    iub.insert('S', 0.02);
    iub.insert('V', 0.02);
    iub.insert('W', 0.02);
    iub.insert('Y', 0.02);
    iub
}

fn make_homosap_hashmap() -> HashMap<char, f64> {
    let mut homosap = HashMap::new();
    homosap.insert('a', 0.3029549426680);
    homosap.insert('c', 0.1979883004921);
    homosap.insert('g', 0.1975473066391);
    homosap.insert('t', 0.3015094502008);
    homosap
}

fn fasta_repeat(count: u32, seq: &str) -> usize {
    let mut seqi: usize = 0;
    let mut len_out: usize = 60;
    let mut ret: usize = 0;

    let mut n = count as i32;
    while n > 0 {
        if (n as usize) < len_out {
            len_out = n as usize;
        }

        if seqi + len_out < seq.len() {
            ret += &seq[seqi .. seqi + len_out].len();
            seqi += len_out;
        } else {
            let s = &seq[seqi..];
            seqi = len_out - s.len();
            ret += format!("{}{}", s, &seq[..seqi]).len();
        }

        n -= len_out as i32;
    }

    ret
}

fn fasta_random(count: u32, table: &mut HashMap<char,f64>) -> usize {
    let mut ret: usize = 0;

    let mut line: Vec<char> = Vec::new();
    line.resize(60, 'x');

    make_cumulative(table);

    let mut n: i32 = count as i32;
    while n > 0 {
        if (n as usize) < line.len() {
            line = Vec::new();
            line.resize(n as usize, 'x');
        }

        for i in 0 .. line.len() {
            let r = rand(1);
            for (key,val) in table.iter() {
                if r < *val {
                    line[i] = *key;
                    break;
                }
            }
        }

        let s: String = line.iter().cloned().collect();
        ret += s.len();
        n -= line.len() as i32;
    }

    ret
}

#[no_mangle]
pub fn string_fasta() -> u32 {
    let mut acc: usize = 0;
    let count = 7;

    acc += fasta_repeat(2*count*100000, ALU);
    acc += fasta_random(3*count*1000, &mut make_iub_hashmap());
    acc += fasta_random(5*count*1000, &mut make_homosap_hashmap());

    acc as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_fasta() {
        assert_eq!(ALU.len(), 287); // Don't accidentally insert newlines or spaces.
        assert_eq!(fasta_repeat(2*7*100000, ALU), 1400000);
        assert_eq!(fasta_random(3*7*1000, &mut make_iub_hashmap()), 21000);
        assert_eq!(fasta_random(5*7*1000, &mut make_homosap_hashmap()), 35000);
        assert_eq!(string_fasta(), 1456000);
    }
}

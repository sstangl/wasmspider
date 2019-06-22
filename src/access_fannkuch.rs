//! Rust implementation of access-fannkuch.

/* The Great Computer Language Shootout
http://shootout.alioth.debian.org/
contributed by Isaac Gouy */

#[no_mangle]
pub fn access_fannkuch(n: usize) -> usize {
    let mut check = 0;
    let mut perm = vec![0; n];
    let mut perm1 = vec![0; n];
    let mut count = vec![0; n];
    let mut max_perm = vec![0; n];
    let mut max_flips_count = 0;
    let m = n - 1;

    for i in 0..n {
        perm1[i] = i;
    }
    let mut r = n;

    loop {
        // write-out the first 30 permutations
        if check < 30 {
            let mut s = String::new();
            for i in 0..n {
                s.push_str(&(perm1[i] + 1).to_string());
            }
            check += 1;
        }

        while r != 1 {
            count[r - 1] = r; // Error: Any array index crashes wasm.
            r -= 1;
        }

        if !(perm1[0] == 0 || perm1[m] == m) {
            for i in 0..n {
                perm[i] = perm1[i];
            }

            let mut flips_count = 0;
            let mut k = perm[0];

            while k != 0 {
                let k2 = (k + 1) >> 1;
                for i in 0..k2 {
                    let temp = perm[i];
                    perm[i] = perm[k - i];
                    perm[k - i] = temp;
                }
                flips_count += 1;
                k = perm[0];
            }

            if flips_count > max_flips_count {
                max_flips_count = flips_count;
                for i in 0..n {
                    max_perm[i] = perm1[i];
                }
            }
        }

        loop {
            if r == n {
                return max_flips_count;
            }
            let perm0 = perm1[0];
            let mut i = 0;
            while i < r {
                let j = i + 1;
                perm1[i] = perm1[j];
                i = j;
            }
            perm1[r] = perm0;

            count[r] = count[r] - 1;
            if count[r] > 0 {
                break;
            }
            r += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_fannkuch() {
        assert_eq!(access_fannkuch(8), 22);
    }
}

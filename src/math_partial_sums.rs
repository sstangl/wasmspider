//! Rust implementation of math-partial-sums.

// The original version is part of The Computer Language Shootout,
// contributed by Isaac Gouy.

fn partial(n: u32) -> f64 {
    let mut _a1: f64 = 0.0;
    let mut _a2: f64 = 0.0;
    let mut _a3: f64 = 0.0;
    let mut _a4: f64 = 0.0;
    let mut _a5: f64 = 0.0;
    let mut a6: f64 = 0.0;
    let mut a7: f64 = 0.0;
    let mut a8: f64 = 0.0;
    let mut a9: f64 = 0.0;

    let twothirds: f64 = 2.0/3.0;
    let mut alt: f64 = -1.0;

    for z in 1..n+1 {
        let k: f64 = z as f64;
        let k2: f64 = k*k;
        let k3: f64 = k2*k;
        let sk: f64 = k.sin();
        let ck: f64 = k.cos();
        alt = -alt;

        _a1 += twothirds.powi((z-1) as i32);
        _a2 += k.powf(-0.5);
        _a3 += 1.0 / (k * (k + 1.0));
        _a4 += 1.0 / (k3 * sk*sk);
        _a5 += 1.0 / (k3 * ck*ck);
        a6 += 1.0 / k;
        a7 += 1.0 / k2;
        a8 += alt / k;
        a9 += alt / (2.0*k - 1.0);
    }

    // Note: Nothing with pow(), sin(), or cos() gets validated, because
    // those aren't well-specified in ECMAScript.
    return a6 + a7 + a8 + a9;
}

#[no_mangle]
pub fn math_partial_sums() -> f64 {
    let mut total: f64 = 0.0;

    let mut i = 1024;
    while i <= 16384 {
        total += partial(i);
        i *= 2;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_partial_sums() {
        assert_eq!(math_partial_sums(), 60.08994194659945);
    }
}

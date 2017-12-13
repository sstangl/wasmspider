//! Rust implementation of controlflow-recursive.
// Original version from The Computer Language Shootout,
// contributed by Isaac Gouy.

fn ack(m: u32, n: u32) -> u32 {
    if m == 0 { return n + 1; }
    if n == 0 { return ack(m-1, 1); }
    ack(m-1, ack(m,n-1))
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n-2) + fib(n-1)
    }
}

fn tak(x: u32, y: u32, z: u32) -> u32 {
    if y >= x { return z; }
    tak(tak(x-1,y,z), tak(y-1,z,x), tak(z-1,x,y))
}

#[no_mangle]
pub fn controlflow_recursive() -> u32 {
    // The JS version uses a global, so mimic that.
    let mut result: Box<u32> = Box::new(0);

    for i in 3..6 {
        *result += ack(3, i);
        *result += fib(17 + i);
        *result += tak(3*i+3, 2*i+2, i+1);
    }

    *result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controlflow_recursive() {
        assert_eq!(controlflow_recursive(), 57775);
    }
}

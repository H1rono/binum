// demonstration: Euclidean Algorithm
use std::{
    cmp,
    io::{self, Write},
};

use binum::UInt;

fn gcd(a: UInt, b: UInt) -> UInt {
    let mut ma = cmp::max(a.clone(), b.clone());
    let mut mi = cmp::min(a, b);
    while mi > UInt::from(0) {
        (ma, mi) = (mi.clone(), ma % mi);
    }
    ma
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut stdout = io::stdout();
    stdout
        .write_all(b"calculate a % b\n")
        .expect("failed to write message");
    stdout.write_all(b"a: ").expect("failed to write message");
    stdout.flush().expect("failed to flush stdout");
    let a = lines
        .next()
        .expect("too few input lines")
        .expect("too few input lines")
        .parse::<u64>()
        .expect("please input number");
    let a = UInt::from(a);
    stdout.write_all(b"b: ").expect("failed to write message");
    stdout.flush().expect("failed to flush stdout");
    let b = lines
        .next()
        .expect("too few input lines")
        .expect("too few input lines")
        .parse::<u64>()
        .expect("please input number");
    let b = UInt::from(b);
    let c = gcd(a, b);
    stdout
        .write_all(format!("gcd(a, b) is {}", u64::from(c)).as_bytes())
        .expect("failed to write message");
}

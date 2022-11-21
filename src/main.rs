// demonstration: ABC210 A - Cabbages
//  https://atcoder.jp/contests/abc210/tasks/abc210_a

use binum::UInt;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lines().next().unwrap().unwrap();
    let (n, a, x, y) = {
        let v = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let f = |i| UInt::from(v[i]);
        (f(0), f(1), f(2), f(3))
    };
    let ans = if n <= a {
        x * n
    } else {
        x * a.clone() + (n - a) * y
    };
    println!("{}", u64::from(ans));
}

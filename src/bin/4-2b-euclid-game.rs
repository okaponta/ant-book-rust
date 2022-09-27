use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        mut a:usize,mut b:usize,
    }
    let mut stan = true;
    loop {
        if a > b {
            swap(&mut a, &mut b);
        }
        if b % a == 0 {
            break;
        }
        if b - a > a {
            break;
        }
        b -= a;
        stan = !stan;
    }
    println!("{}", if stan { "Stan" } else { "Ollie" })
}

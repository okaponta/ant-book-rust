use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        k:usize,
    }
    println!("{}", a.lower_bound(&k));
}

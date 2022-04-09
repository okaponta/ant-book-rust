use proconio::input;

fn main() {
    input! {
        n:usize,
        mut s:String,
    }
    let mut srev = s.chars().rev().collect::<String>();
    let mut t = vec![];
    for _ in 0..n {
        let c;
        if s.cmp(&srev).is_le() {
            c = s.remove(0);
            srev.pop();
        } else {
            c = srev.remove(0);
            s.pop();
        }
        t.push(c);
    }
    println!("{}", t.iter().collect::<String>());
}

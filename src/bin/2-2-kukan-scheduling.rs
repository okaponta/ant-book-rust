use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[usize;n],
        t:[usize;n],
    }
    let mut st: Vec<_> = s.iter().zip(t.iter()).collect();
    st.sort_by_key(|e| e.1);
    let mut count = 0;
    let mut time = 0;
    for i in 0..n {
        if &time < st[i].0 {
            count += 1;
            time = *st[i].1;
        }
    }
    println!("{}", count);
}

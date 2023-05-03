use proconio::input;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut st = vec![];

    let mut l = vec![0; n];
    for i in 0..n {
        while 0 < st.len() && h[i] <= h[st.len() - 1] {
            st.pop();
        }
        l[i] = if st.len() == 0 {
            0
        } else {
            st[st.len() - 1] + 1
        };
        st.push(i);
    }

    st = vec![];
    let mut r = vec![0; n];
    for i in (0..n).rev() {
        while 0 < st.len() && h[i] <= h[st[st.len() - 1]] {
            st.pop();
        }
        r[i] = if st.len() == 0 { n } else { st[st.len() - 1] };
        st.push(i);
    }

    let mut res = 0;
    for i in 0..n {
        res = res.max(h[i] * (r[i] - l[i]));
    }
    println!("{}", res);
}

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        st:[(usize,usize);m],
    }
    let mut edge = vec![vec![]; n];
    for (s, t) in st {
        edge[s].push(t);
        edge[t].push(s);
    }
    // 0が初期値。1と-1で彩色してく
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            if !dfs(i, 1, &mut color, &edge) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

fn dfs(cur: usize, col: i32, color: &mut Vec<i32>, edge: &Vec<Vec<usize>>) -> bool {
    color[cur] = col;
    for &next in &edge[cur] {
        if color[next] == col {
            return false;
        }
        if color[next] == 0 {
            if !dfs(next, -col, color, edge) {
                return false;
            }
        }
    }
    true
}

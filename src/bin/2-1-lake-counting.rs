use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:i32,
        m:i32,
        mut s:[Chars;n],
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if s[i as usize][j as usize] == 'W' {
                dfs(i, j, n, m, &mut s);
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn dfs(x: i32, y: i32, n: i32, m: i32, s: &mut Vec<Vec<char>>) {
    s[x as usize][y as usize] = '.';
    for dx in -1..=1 {
        for dy in -1..=1 {
            let nx = x + dx;
            let ny = y + dy;
            if 0 <= nx && nx < n && 0 <= ny && ny < m && s[nx as usize][ny as usize] == 'W' {
                dfs(nx, ny, n, m, s);
            }
        }
    }
}

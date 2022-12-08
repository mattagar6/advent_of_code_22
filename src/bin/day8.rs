use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn solve() {
    let grid: Vec<Vec<i32>> = read_lines("day8.in")
        .into_iter()
        .map(|x| x.chars().map(|ch| ch as i32 - '0' as i32).collect())
        .collect();

    fn get_score(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> u64 {
        let mut ans = 1;
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;

        fn valid(i: i32, j: i32, n: i32, m: i32) -> bool {
            0 <= i && i < n && 0 <= j && j < m
        }

        let dr = [-1,0,0,1];
        let dc = [0,-1,1,0];

        for k in 0..4 {
            let mut r = i as i32;
            let mut c = j as i32;
            let mut cur = 0;
            while valid(r + dr[k], c + dc[k], n, m) && grid[(r + dr[k]) as usize][(c + dc[k]) as usize] < grid[i][j] {
                cur += 1;
                r += dr[k];
                c += dc[k];
            }
            if valid(r + dr[k], c + dc[k], n, m) {
                cur += 1;
            }
            ans *= cur;
        }
        return ans;
    }

    let ans = grid.iter().enumerate().map(|(i, row)|
                    row.iter().enumerate().map(|(j, _)| get_score(&grid, i, j)).max().unwrap())
                    .max().unwrap();
    println!("ans = {}", ans);
}

fn main() {
    solve();
}
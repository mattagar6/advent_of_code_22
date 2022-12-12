use std::{fs, collections::VecDeque};

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn solve() {
    let lines = read_lines("day12.in");
    let n = lines.len();
    let m = lines[0].len();
    let mut grid: Vec<Vec<char>> = vec![Default::default(); n];
    for i in 0..n {
        grid[i] = lines[i].chars().collect();
    }
    let mut dist: Vec<Vec<i32>> = vec![vec![(n*m) as i32; m]; n];
    
    let mut qu: VecDeque<(usize,usize)> = Default::default();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'S' || grid[i][j] == 'a' {
                grid[i][j] = 'a';
                dist[i][j] = 0;
                qu.push_back((i, j));
            }
        }
    }

    assert!(!qu.is_empty());

    let n = n as i32;
    let m = m as i32;
    
    fn valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        0 <= i && i < n && 0 <= j && j < m
    }

    let dr = [-1, 0, 0, 1];
    let dc = [0, -1, 1, 0];

    while !qu.is_empty() {
        let (i, j) = *qu.front().unwrap();
        qu.pop_front();
        if grid[i][j] == 'E' {
            let d = dist[i][j];
            println!("ans = {d}");
        }
        for k in 0..4 {
            let r = i as i32 + dr[k];
            let c = j as i32 + dc[k];
            if !valid(r, c, n, m) {
                continue;
            }
            let r = r as usize;
            let c = c as usize;
            let val = if grid[r][c] == 'E' { 'z' } else { grid[r][c] };
            if val as i32 - grid[i][j] as i32 <= 1 && dist[i][j] + 1 < dist[r][c] {
                dist[r][c] = dist[i][j] + 1;
                qu.push_back((r, c));
            } 
        }
    }

}

fn main() {
    solve();
}
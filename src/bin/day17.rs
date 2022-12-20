use std::fs;
use std::cmp::{max, min};

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn solve() {
    let lines = read_lines("day17.in");
    assert!(lines.len() == 1);
    let moves: Vec<char> = lines[0].chars().collect();

    const N: usize = 2022 * 8;
    const M: usize = 7;

    let mut grid = [['.'; M]; N];
    let mut mark = [[0; M]; N];
    let mut memo = [[0; N]; 5];
    let mut last = [[0; N]; 5];

    let mut count = [[0; N]; 5];
    let mut next_rock = [[0; N]; 5];
    let mut next_t = [[0; N]; 5];
    let mut delta = [[0; N]; 5];


    let mut row = N - 4;
    let mut t = 0;
    let mut prev_rock = 0;
    let mut prev_t = 0;

    let mut tot_rocks: i64 = 1000000000000;



    fn left(grid: &mut [[char; M]; N], mark: &mut [[i32; M]; N], top: usize, bot: usize) {
        let mut can = true;
        for row in top..bot {
            for col in 0..M {
                if mark[row][col] == 1 {
                    if col == 0 || mark[row][col-1] == 2 {
                        can = false;
                    }
                }
            }
        }
        if can {
            for row in top..bot {
                for col in 0..M {
                    if mark[row][col] == 1 {
                        mark[row][col-1] = 1;
                        grid[row][col-1] = 'x';
                        mark[row][col] = 0;
                        grid[row][col] = '.';
                    }
                }
            }
        }
    }

    fn right(grid: &mut [[char; M]; N], mark: &mut [[i32; M]; N], top: usize, bot: usize) {
        let mut can = true;
        for row in top..bot {
            for col in 0..M {
                if mark[row][col] == 1 {
                    if col == M-1 || mark[row][col+1] == 2 {
                        can = false;
                    }
                }
            }
        }
        if can {
            for row in top..bot {
                for col in (0..M).rev() {
                    if mark[row][col] == 1 {
                        mark[row][col+1] = 1;
                        grid[row][col+1] = 'x';
                        mark[row][col] = 0;
                        grid[row][col] = '.';
                    }
                }
            }
        }
    }

    fn down(grid: &mut [[char; M]; N], mark: &mut [[i32; M]; N], top: usize, bot: usize) -> bool {
        let mut can = true;
        for row in top..bot {
            for col in 0..M {
                if mark[row][col] == 1 {
                    if row == N-1 || mark[row + 1][col] == 2 {
                        can = false;
                    }
                }
            }
        }
        if !can {
            return false;
        }

        for row in (top..bot).rev() {
            for col in 0..M {
                if mark[row][col] == 1 {
                    mark[row + 1][col] = 1;
                    grid[row + 1][col] = 'x';
                    mark[row][col] = 0;
                    grid[row][col] = '.';
                }
            }
        }
        
        return true;
    }

    for rock in 0..20*1000 {
        // row is top of the rock.
        println!("Doing rock: {rock}");
        match rock % 5 {
            0 => {
                for col in 2..6 {
                    grid[row][col] = 'x';
                    mark[row][col] = 1;
                }
                row -= 1;
            }
            1 => {
                grid[row][3] = 'x';
                mark[row][3] = 1;
                row -= 1;
                for col in 2..5 {
                    grid[row][col] = 'x';
                    mark[row][col] = 1;
                }
                row -= 1;
                grid[row][3] = 'x';
                mark[row][3] = 1;
                row -= 1;
            }
            2 => {
                for col in 2..5 {
                    grid[row][col] = 'x';
                    mark[row][col] = 1;
                }
                row -= 1;
                for _ in 0..2 {
                    grid[row][4] = 'x';
                    mark[row][4] = 1;
                    row -= 1;
                }
            }
            3 => {
                for _ in 0..4 {
                    grid[row][2] = 'x';
                    mark[row][2] = 1;
                    row -= 1;
                }
            }
            4 => {
                for _ in 0..2 {
                    for col in 2..4 {
                        grid[row][col] = 'x';   
                        mark[row][col] = 1;
                    }
                    row -= 1;
                }
            }
            _ => {
                assert!(false);
            }
        }
        
        let mut top = max(0, row - 10) as usize;
        let mut bot = min(N, row + 10) as usize;

        let start_t = t % moves.len();



        loop {
            let ch = moves[t % moves.len()];
            if ch == '>' {
                right(&mut grid, &mut mark, top, bot);
            }
            if ch == '<' {
                left(&mut grid, &mut mark, top, bot);
            }
            t += 1;
            if !down(&mut grid, &mut mark, top, bot) {
                break;
            }
            top += 1;
            bot += 1;
            bot = min(bot, N);
        }



        // update row.
        for i in (0..N).rev() {
            for j in 0..M {
                if grid[i][j] == 'x' {
                    mark[i][j] = 2;
                    row = i;
                }
            }
        }
        let h = N - row;
        let r = rock % 5;
        next_rock[prev_rock][prev_t] = r;
        next_t[prev_rock][prev_t] = start_t;
        delta[prev_rock][prev_t] = h - last[prev_rock][prev_t];
        prev_rock = r;
        prev_t = start_t;
        tot_rocks -= 1;


        if count[r][start_t] > 5 {
            println!("{r}");
            let a = h - last[r][start_t];
            let b = memo[r][start_t];
            println!("{a} {b}");
            assert!(h - last[r][start_t] == memo[r][start_t]);
            let start_h = h;
            let mut len: i64 = 0;
            let mut i = r;
            let mut j = start_t;

            let mut vis = [[0; N]; 5];
            while vis[i][j] == 0 {
                len += 1;
                vis[i][j] = 1;
                let new_i = next_rock[i][j];
                let new_j = next_t[i][j];
                i = new_i;
                j = new_j;
            }
            let mut ans: i64 = start_h as i64;
            ans += (tot_rocks / len) * a as i64;
            tot_rocks %= len;
            while tot_rocks > 0 {
                ans += delta[i][j] as i64;
                let new_i = next_rock[i][j];
                let new_j = next_t[i][j];
                i = new_i;
                j = new_j;
                tot_rocks -= 1;
            }

            println!("ANS = {ans}");

            // do ans here
            break;
        }
        count[r][start_t] += 1;
        memo[r][start_t] = h - last[r][start_t];
        last[r][start_t] = h;

        row -= 4;
    }

    let mut ans = N;
    for i in (0..N).rev() {
        for j in 0..M {
            if grid[i][j] == 'x' {
                ans = i;
            }
        }
    }

    ans = N - ans;
    println!("ans = {ans}");



}

fn main() {
    solve();
}
use std::fs;
#[macro_use] extern crate scan_fmt;

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}
/*
                    [L]     [H] [W]
                [J] [Z] [J] [Q] [Q]
[S]             [M] [C] [T] [F] [B]
[P]     [H]     [B] [D] [G] [B] [P]
[W]     [L] [D] [D] [J] [W] [T] [C]
[N] [T] [R] [T] [T] [T] [M] [M] [G]
[J] [S] [Q] [S] [Z] [W] [P] [G] [D]
[Z] [G] [V] [V] [Q] [M] [L] [N] [R]
 1   2   3   4   5   6   7   8   9 

*/

fn init() -> [Vec<char>; 10] {
    let mut stk: [Vec<char>; 10] = Default::default();
    stk[1] = vec!['Z', 'J', 'N', 'W', 'P', 'S'];
    stk[2] = vec!['G', 'S', 'T'];
    stk[3] = vec!['V', 'Q', 'R', 'L', 'H'];
    stk[4] = vec!['V', 'S', 'T', 'D'];
    stk[5] = vec!['Q', 'Z', 'T', 'D', 'B', 'M', 'J'];
    stk[6] = vec!['M', 'W', 'T', 'J', 'D', 'C', 'Z', 'L'];
    stk[7] = vec!['L', 'P', 'M', 'W', 'G', 'T', 'J'];
    stk[8] = vec!['N', 'G', 'M', 'T', 'B', 'F', 'Q', 'H'];
    stk[9] = vec!['R', 'D', 'G', 'C', 'P', 'B', 'Q', 'W'];
    stk
}

fn part1() {
    
    let mut stk = init();
    while let Ok((amt, from, to)) = scanln_fmt!("move {} from {} to {}", usize, usize, usize) {
        for _ in 0..amt {
            let x = stk[from].pop().unwrap();
            stk[to].push(x);
        }
    }
    for i in 1..10 {
        let top = stk[i].pop().unwrap();
        print!("{top}");
    }
    println!("");
}

fn part2() {
        
    let mut stk = init();
    while let Ok((amt, from, to)) = scanln_fmt!("move {} from {} to {}", usize, usize, usize) {
        let mut vec = Vec::<char>::new();
        for _ in 0..amt {
            let x = stk[from].pop().unwrap();
            vec.push(x);
        }
        vec.reverse();
        for x in vec {
            stk[to].push(x);
        }
    }
    for i in 1..10 {
        let top = stk[i].pop().unwrap();
        print!("{top}");
    }
    println!("");
}

fn main() {
    // part1();
    part2();
}
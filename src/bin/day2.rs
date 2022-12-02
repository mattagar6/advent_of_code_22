use std::{io::{self, BufRead}};
use std::cmp::max;

fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut vec = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        // println!("{line}");
        vec.push(line);
    }
    vec
}

fn score(you: char, me: char) -> u32 {
    let x = you as u32 - 'A' as u32;
    let y = me as u32 - 'X' as u32;
    let win = if x == y { 3 } else if (x + 1) % 3 == y { 6 } else { 0 };
    win + y + 1
}

fn part1() {
    let mut ans = 0;
    for line in read_lines() {
        assert!(line.len() == 3);
        ans += score(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
    }
    println!("Score = {ans}");
}

fn parse(s: String) -> (usize, usize) {
    (s.chars().nth(0).unwrap() as usize - 'A' as usize, s.chars().nth(2).unwrap() as usize - 'X' as usize)
}

fn part2() {
    // let mut games = Vec::<(char,char)>::new();
    let mut games = [[0 ; 3] ; 3]; // for fun, lets create a 2d array
    for line in read_lines() {
        let (x, y) = parse(line);
        games[x][y] += 1;
    }

    let mut ans = 0;
    for him in 0..3 {
        for res in 0..3 {
            // 0 -> -1, 1 -> 0, 2 -> 1
            let me = (him + res + 3 - 1) % 3;
            ans += (me + 1 + res * 3) * games[him][res];
        }
    }

    println!("Score = {ans}");

}

fn main() {
    // part1();
    part2();
}
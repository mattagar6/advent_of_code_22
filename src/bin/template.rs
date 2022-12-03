use std::{io::{self, BufRead}};

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

fn part1() {

}

fn part2() {
    
}

fn main() {
    // part1();
    // part2();
}
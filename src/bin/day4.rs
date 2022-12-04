use std::{io::{self, BufRead}};
use std::cmp::{min, max};
#[macro_use] extern crate scan_fmt;


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

fn inside(l1: i32, r1: i32, l2: i32, r2: i32) -> bool {
    (l1 <= l2 && r2 <= r1) || (l2 <= l1 && r1 <= r2) 
}

fn part1() {
    let mut ans = 0;
    while let Ok((l1,r1,l2,r2)) = scanln_fmt!("{}-{},{}-{}", i32, i32, i32, i32) {
        ans += if inside(l1,r1,l2,r2) { 1 } else { 0 }
    }
    println!("Part1 = {ans}");
}

fn overlap(l1: i32, r1: i32, l2: i32, r2: i32) -> bool {
    max(l1, l2) <= min(r1, r2)
}

fn part2() {
    let mut ans = 0;
    while let Ok((l1,r1,l2,r2)) = scanln_fmt!("{}-{},{}-{}", i32, i32, i32, i32) {
        ans += if overlap(l1,r1,l2,r2) { 1 } else { 0 }
    }
    println!("Part2 = {ans}");
}

fn main() {
    // part1();
    part2();
}
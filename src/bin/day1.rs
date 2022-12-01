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

fn part1() {
    let lines = read_lines();
    let mut most = 0;
    let mut cur = 0;
    for line in lines {
        if line.is_empty() {
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
        most = max(most, cur);
    }
    println!("Most = {most}");
}

fn part2() {
    let lines = read_lines();
    let mut cals = Vec::<i32>::new();
    let mut cur = 0;
    for line in lines {
        if line.is_empty() {
            cals.push(cur);
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }
    cals.push(cur);
    cals.sort();
    cals.reverse();
    let tot = {
        let mut sum = 0;
        for i in 0..3 {
            sum += cals[i];
        }
        sum
    };
    println!("Sum of Cals = {tot}");
}

fn main() {
    // part1();
    part2();
}
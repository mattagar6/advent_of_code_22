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

fn priority(ch: char) -> u32 {
    if ch.is_lowercase() { ch as u32 - 'a' as u32 + 1 } else { ch as u32 - 'A' as u32 + 27 }
}

fn part1() {
    let mut ans = 0;
    for line in read_lines() {
        let len = line.len() / 2;
        assert!(line.len() % 2 == 0);
        let s = &line[..len];
        let t = &line[len..];
        for ch in s.as_bytes() {
            if t.contains(*ch as char) {
                ans += priority(*ch as char);
                break;
            }
        }
    }
    println!("Sum of priorities = {ans}");
}

fn part2() {
    // lets use some bitmasks :)

    let lines = read_lines();
    assert!(lines.len() % 3 == 0);
    let mut ans = 0;

    for i in 0..lines.len()/3 {
        let mut mask: u64 = u64::MAX;
        for j in 0..3 {
            let mut now: u64 = 0;
            for ch in lines[3*i+j].as_bytes() {
                now |= (1 as u64) << priority(*ch as char);
            }
            mask &= now;
        }
        assert!(mask.count_ones() == 1);
        while mask % 2 == 0 {
            ans += 1;
            mask /= 2;
        }
    }
    println!("Sum of priorities 2 = {ans}");

}

fn main() {
    // part1();
    part2();
}
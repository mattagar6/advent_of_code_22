use std::{fs, collections::HashSet, hash::Hash};
use std::num;

#[macro_use] extern crate scan_fmt;

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn solve() {
    let mut vis: HashSet<(i32, i32)> = HashSet::new();
    vis.insert((0,0));
    let mut r = [0; 10];
    let mut c = [0; 10];
    while let Ok((dir, amt)) = scanln_fmt!("{} {}", char, i32) {
        // println!("Doing: {dir}, {amt}");
        let (dx, dy) = match dir {
            'L' => {(-1, 0)}
            'R' => {(1, 0)}
            'U' => {(0, -1)}
            'D' => {(0, 1)}
            _ => {
                assert!(false);
                (0,0)
            }
        };

        for _ in 0..amt { // invariant: r[1], c[1] is in vis
            r[0] += dx;
            c[0] += dy;
            for i in 1..10 {
                let dist = i32::abs(r[i-1] - r[i]) + i32::abs(c[i-1] - c[i]);
                if dist <= 1 {
                    continue;
                }
                if dist == 2 && r[i-1] != r[i] && c[i-1] != c[i] {
                    continue; // hacky but whatever
                }
                // should be 1 unique direction that brings it to 1
                let mut found = false;
                'outer: for dx in -1..2 {
                    for dy in -1..2 {
                        if i32::abs(r[i-1] - (r[i] + dx)) + i32::abs(c[i-1] - (c[i] + dy)) == 1 {
                            found = true;
                            r[i] += dx;
                            c[i] += dy;
                            break 'outer;
                        }
                    }
                }
                if !found {
                    assert!(dist == 4);
                    assert!(i32::abs(r[i] - r[i-1]) == 2 && i32::abs(c[i] - c[i-1]) == 2);
                    r[i] += (r[i-1] - r[i])/2;
                    c[i] += (c[i-1] - c[i])/2;
                }
            }
            vis.insert((r[9], c[9]));
        }
    }
    let ans = vis.len();
    println!("ans = {ans}");
}

fn main() {
    solve();
}
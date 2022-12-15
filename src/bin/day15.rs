use std::{collections::{HashMap, HashSet}};

use::std::cmp::max;
#[macro_use] extern crate scan_fmt;

fn consider_row(ranges: &mut Vec<(i32, i32)>, y: i32) {
    if y < 0 || y > 4000000 {
        return;
    }
    ranges.push((i32::MIN, i32::MIN));
    ranges.push((i32::MAX, i32::MAX));
    ranges.sort_by(|a,b| a.0.cmp(&b.0));
    assert!(ranges.len() > 1);
    let mut max_r = i32::MIN;
    for (l, r) in ranges {
        if *l != i32::MIN {
            let x = *l - 1;
            if x > max_r && x >= 0 && x <= 4000000 {
                let score: i64 = (x as i64) * 4000000 + (y as i64);
                println!("ans = ({x}, {y}) -> {score}");
            }
        }
        max_r = max(max_r, *r);
    }
}

fn solve() {
    let mut pts: Vec<(i32, i32, i32)> = Default::default();
    let mut count_row: HashMap<i32, i32> = Default::default();
    let mut vis: HashSet<(i32, i32)> = Default::default();
    while let Ok((x1, y1, x2, y2)) = scanln_fmt!("Sensor at x={}, y={}: closest beacon is at x={}, y={}", i32, i32, i32, i32) {
        pts.push((x1, y1, (x1 - x2).abs() + (y1 - y2).abs()));
        if !vis.contains(&(x2, y2)) {
            vis.insert((x2, y2));
            *count_row.entry(y2).or_default() += 1;

        }
    }

    let mut ranges: HashMap<i32, Vec<(i32, i32)>> = Default::default();
    for (x, y, d) in pts {
        for y2 in y-d..y+d+1 {
            ranges.entry(y2).or_default().push((x-(d-(y-y2).abs()), x+d-(y-y2).abs()));
        }
    }

    for (y, mut row) in ranges {
        consider_row(&mut row, y);
    }
}

fn main() {
    solve();
}
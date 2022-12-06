use std::{fs, collections::HashMap};

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn part1() {
    let s: Vec<char> = read_lines("day6.in")[0].chars().collect(); 
    let mut j = 0;
    let mut mp: HashMap<char, i32> = HashMap::new();
    for i in 0..s.len()-4+1 {
        while j < i + 4 {
            *mp.entry(s[j]).or_default() += 1;
            j += 1;
        }
        let mut ok = true;
        for (k, v )in &mp {
            if *v > 1 {
                ok = false;
            }
        }
        *mp.entry(s[i]).or_default() -= 1;
        if ok {
            let ans = i + 4;
            println!("Part 1 ans = {ans}");
            return;
        }
    }
    assert!(false);
}

fn part2() {
    let s: Vec<char> = read_lines("day6.in")[0].chars().collect(); 
    let mut j = 0;
    let mut mp: HashMap<char, i32> = HashMap::new();
    for i in 0..s.len()-14+1 {
        while j < i + 14 {
            *mp.entry(s[j]).or_default() += 1;
            j += 1;
        }
        let mut ok = true;
        for (k, v )in &mp {
            if *v > 1 {
                ok = false;
            }
        }
        *mp.entry(s[i]).or_default() -= 1;
        if ok {
            let ans = i + 14;
            println!("Part 1 ans = {ans}");
            return;
        }
    }
    assert!(false);
}

fn main() {
    // part1();
    part2();
}
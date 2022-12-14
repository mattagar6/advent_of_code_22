use std::{fs, collections::HashSet, cmp::min, cmp::max};

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn solve() {
    let lines = read_lines("day14.in");
    let mut vis: HashSet<(i32, i32)> = Default::default();
    let mut big = 0;
    for line in lines {
        let path: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();
        let n = path.len();
        assert!(n > 1);
        for i in 0..n-1 {
            let p: Vec<i32> = path[i].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            let q: Vec<i32> = path[i+1].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            assert!(p.len() == 2);
            assert!(q.len() == 2);
            assert!(p[0] == q[0] || p[1] == q[1]);
            big = max(big, max(p[1], p[1]) + 1);
            for x in min(p[0], q[0])..max(p[0], q[0])+1 {
                vis.insert((x, p[1]));
            }
            for y in min(p[1], q[1])..max(p[1], q[1])+1 {
                vis.insert((p[0], y));
            }
        }
    }

    let mut ans = 0;

    let ord = [0, -1, 1];

    while !vis.contains(&(500,0)) {
        

        let mut r = 0;
        let mut c = 500;

        'outer: while r <= big {
            assert!(!vis.contains(&(c, r)));
            for dc in ord {
                if !vis.contains(&(c + dc, r + 1)) && r < big {
                    r += 1;
                    c += dc;
                    continue 'outer;
                }
            }

            // println!("{ans}: {r}, {c}");
            ans += 1;
            vis.insert((c, r));
            break;
        } 
    }

    println!("ans = {ans}");



}

fn main() {
    solve();
}
use std::{collections::{HashSet, VecDeque}};

#[macro_use] extern crate scan_fmt;

fn main() {
    let mut st: HashSet<(i32, i32, i32)> = Default::default();
    while let Ok((x, y, z)) = scanln_fmt!("{},{},{}", i32, i32, i32) {
        st.insert((x, y, z));
    }

    let mut ans = 0;
    let dx = [-1, 1, 0, 0, 0, 0];
    let dy = [0, 0, -1, 1, 0, 0];
    let dz = [0, 0, 0, 0, -1, 1];

    let mut qu: VecDeque<(i32, i32, i32)> = Default::default();
    let mut vis: HashSet<(i32, i32, i32)> = Default::default();

    qu.push_back((0,0,0));
    vis.insert((0,0,0));    

    while !qu.is_empty() {
        let (x, y, z) = *qu.front().unwrap();
        qu.pop_front();
        if x < -100 || y < -100 || z < -100 || x > 100 || y > 100 || z > 100 || st.contains(&(x, y, z)) {
            continue;
        }
        

        for k in 0..6 {
            if !vis.contains(&(x + dx[k], y + dy[k], z + dz[k])) {
                qu.push_back((x + dx[k], y + dy[k], z + dz[k]));
                vis.insert((x + dx[k], y + dy[k], z + dz[k]));
            }
        }
    }

    for (x, y, z) in &st {
        let x = *x;
        let y = *y;
        let z = *z;
        if vis.contains(&(x, y, z)) {
            for k in 0..6 {
                if !st.contains(&(x + dx[k], y + dy[k], z + dz[k])) && vis.contains(&(x + dx[k], y + dy[k], z + dz[k])) {
                    ans += 1;
                }
            }
        }
    }

    println!("ans = {ans}");


}
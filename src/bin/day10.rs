use std::{fs};


fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn solve() {
    const N: usize = 1000;
    let mut val: [i32; 2*N] = [0; 2*N];
    let mut X = 1;
    let mut cyc = 1; 
    let mut grid: [[char; 40]; 6] = [['.'; 40]; 6];

    for line in read_lines("day10.in") {
        let tok: Vec<&str> = line.split(" ").collect();

        match tok[0] {
            "addx" => {
                let amt = tok[1].parse::<i32>().unwrap();
                for _ in 0..2 {
                    val[cyc] = X;
                    if i32::abs(val[cyc] - ((cyc as i32) - 1) % 40) <= 1 {
                        let row = (cyc-1)/40;
                        let col = (cyc-1)%40;
                        grid[row][col] = '#';
                    }
                    cyc += 1;
                }
                X += amt;
                val[cyc] = amt;
                // if i32::abs(val[cyc] - ((cyc as i32) - 1) % 40) <= 1 {
                //     let row = (cyc-1)/40;
                //     let col = (cyc-1)%40;
                //     grid[row][col] = '#';
                // }
            }
            _ => {
                val[cyc] = X;
                if i32::abs(val[cyc] - ((cyc as i32) - 1) % 40) <= 1 {
                    let row = (cyc-1)/40;
                    let col = (cyc-1)%40;
                    grid[row][col] = '#';
                }
                cyc += 1;
            }
        }
        
    }
    
    for row in 0..6 {
        let s: String = grid[row].iter().collect();
        println!("{s}");
    }
    

    // for i in 1..10 {
    //     let amt = val[i];
    //     println!("{i} = {amt}");
    // }
    // let days = [20,60,100,140,180,220];
    // for i in days {
    //     let amt = val[i];
    //     println!("{i} = {amt}");
    // }
    // let ans: i32 = days.map(|x| { val[x] * (x as i32) }).into_iter().sum();
    // println!("ans = {ans}");

}

fn main() {
    solve();
}
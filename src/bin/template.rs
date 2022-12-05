use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn part1() {

}

fn part2() {
    
}

fn main() {
    // part1();
    // part2();
}
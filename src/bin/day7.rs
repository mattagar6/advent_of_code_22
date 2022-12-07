use std::{fs::{self}, collections::{HashMap}, cmp::min};

fn read_lines(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("Read Input");
    content.lines().map(|line| { line.to_string() }).collect()
}

fn dfs(v: String, child: &HashMap<String,Vec<String>>, dir_size: &mut HashMap<String,u64>) {
    if child.contains_key(&v) {
        for u in &child[&v] {
            dfs(u.to_string(), child, dir_size);
            // why is this so ugly? :( I'm probably doing something wrong.
            *dir_size.entry(v.to_string()).or_default() += *dir_size.entry(u.to_string()).or_default();
        }
    }
}

fn solve() {
    // let lines = read_lines("small.in");
    let lines = read_lines("day7.in");
    let n = lines.len();
    let mut child: HashMap<String,Vec<String>> = HashMap::new();
    let mut dir_size: HashMap<String,u64> = HashMap::new();

    let mut cur: String = String::from("/");
    for i in 0..n {
        let line = lines[i].to_string();
        if line.starts_with("$") {
            assert!(cur.ends_with("/"));
            let tok: Vec<&str> = line.split(" ").collect();
            match tok[1] {
                "cd" => {
                    let new_dir = tok[2];
                    match new_dir {
                        ".." => {
                            let pos = cur[..cur.len()-1].rfind("/").unwrap();
                            cur = cur[..pos + 1].to_string();
                        }
                        "/" => {
                            cur = String::from("/");
                        }
                        other => {
                            cur.push_str(other);
                            cur.push('/');
                        }
                    }
                }
                "ls" => {
                    if child.contains_key(&cur) {
                        // we did ls already.
                        continue;
                    }
                    for j in i+1..n {
                        if lines[j].starts_with("$") {
                            break;
                        }
                        let entry: Vec<&str> = lines[j].split(" ").collect();
                        let dir_name = cur.to_string() + entry[1] + "/";
                        match entry[0] {
                            "dir" => {
                                child.entry(cur.to_string()).or_default().push(dir_name);
                            }
                            size => {
                                *dir_size.entry(cur.to_string()).or_default() += size.parse::<u64>().unwrap();
                            }
                        }
                    }
                }
                _ => assert!(false),
            }
        }
    }

    dfs(String::from("/"), &child, &mut dir_size);
    let tot_size: u64 = 70000000;
    let avail = tot_size - dir_size[&String::from("/")];
    println!("Avail space: {avail}");
    let mut ans: u64 = u64::MAX;
    for (_, v) in &dir_size {
        if avail + *v >= 30000000 {
            ans = min(ans, *v);
        }
    }

    println!("Ans = {ans}");
}

fn main() {
    solve();
}
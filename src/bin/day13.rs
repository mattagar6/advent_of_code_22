use std::fs;
use std::cmp::Ordering;

fn read_file(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("Read Input");
    content
}

fn find_last(s: &Vec<char>, b: usize) -> usize {
    let mut b = b;
    let mut bal = 0;
    loop {
        if s[b] == '[' {
            bal += 1;
        } else if s[b] == ']' {
            bal -= 1;
        }
        if bal == 0 {
            return b;
        }
        assert!(bal > 0);
        b += 1;
    }
}

fn rec(s: &Vec<char>, b: usize, e: usize, t: &Vec<char>, l: usize, r: usize) -> Ordering {


    if b < e && s[b] == ',' {
        return rec(s, b + 1, e, t, l, r);
    }

    if l < r && t[l] == ',' {
        return rec(s, b, e, t, l + 1, r);
    }

    if b >= e {
        return if l >= r {Ordering::Equal} else {Ordering::Less};
    }

    if l >= r {
        return Ordering::Greater;
    }
    assert!(s[b] != ']');
    assert!(t[l] != ']');

    if s[b] == '[' && t[l] == '[' {
        let ee = find_last(s, b);
        let rr = find_last(t, l);
        let res = rec(s, b + 1, ee, t, l + 1, rr);
        if res != Ordering::Equal {
            return res;
        }
        return rec(s, ee + 1, e, t, rr + 1, r);
    }

    if s[b] == '[' {
        let ee = find_last(s, b);
        let res = rec(s, b + 1, ee, t, l, l + 1); // t becomes single element list
        if res != Ordering::Equal {
            return res;
        }
        return rec(s, ee + 1, e, t, l + 1, r);
    }

    if t[l] == '[' {
        let rr = find_last(t, l);
        let res = rec(s, b, b+1, t, l + 1, rr);
        if res != Ordering::Equal {
            return res;
        }
        return rec(s, b+1, e, t, rr + 1, r);
    }

    if s[b] == t[l] {
        return rec(s, b+1, e, t, l+1, r);
    }

    return if s[b] < t[l] {Ordering::Less} else {Ordering::Greater};
}

fn solve() {
    let mut raw_input = read_file("day13.in");
    raw_input.push_str("\n[[2]]");//[[c]]
    raw_input.push_str("\n[[6]]");//[[g]]
    for x in (0..26).rev() {
        let ch = 'a' as u32 + x as u32;
        let ch = char::from_u32(ch).unwrap();
        raw_input = raw_input.replace(&x.to_string(), &ch.to_string());
    }
    raw_input = raw_input.replace("\n\n", "\n");
    // print!("{raw_input}");
    // assert!('a' < 'b');// Yay I can compare chars.

    let mut packets: Vec<Vec<char>> = raw_input.split("\n").map(|line| { line.to_string().chars().collect() }).collect();
    
    packets.sort_by(|a,b| {
        rec(&a, 0, a.len(), &b, 0, b.len())
    });

    let lines: Vec<String> = packets.into_iter().map(|s| s.into_iter().collect()).collect();

    let c = lines.iter().position(|s| s == "[[c]]").unwrap();
    let g = lines.iter().position(|s| s == "[[g]]").unwrap();

    let ans = (c + 1) * (g + 1);
    println!("ans = {ans}");

    // for x in packets {
    //     let line: String = x.into_iter().collect();
    //     println!("{line}");
    // }

}

fn main() {
    solve();
}
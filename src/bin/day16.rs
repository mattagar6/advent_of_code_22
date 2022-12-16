use std::{collections::HashMap};
use std::cmp::max;

#[macro_use] extern crate scan_fmt;

fn main() {

    let mut flow: HashMap<usize, i32> = Default::default();
    let mut adj: HashMap<usize, Vec<String>> = Default::default();
    let mut spec: Vec<String> = Default::default();
    let mut id: HashMap<String, usize> = Default::default();
    let mut spec_id: HashMap<usize, usize> = Default::default();

    while let Ok((v, rate, nei)) = scanln_fmt!("Valve {} has flow rate={}; tunnels lead to valves {}", String, i32, String) {
        flow.insert(id.len(), rate);
        for u in nei.split(",") {
            adj.entry(id.len()).or_default().push(u.to_string());
        }
        if rate > 0 {
            spec_id.insert(id.len(), spec.len());
            spec.push(v.to_owned());
        }
        id.insert(v.to_owned(), id.len());

    }

    spec_id.insert(*id.get("AA").unwrap(), spec.len());
    spec.push("AA".to_string());

    let n = spec.len();
    let tim = (26-1)*2;

    let mut dp = vec![vec![vec![i32::MIN; id.len()]; 1 << n]; tim + 1];
    dp[0][1 << *spec_id.get(&*id.get("AA").unwrap()).unwrap()][*id.get("AA").unwrap()] = 0;

    for t in 0..tim {
        for mask in 0..((1<<n) as usize) {
            for mut v in 0..id.len() {
                if dp[t][mask][v] < 0 {
                    continue;
                }
                let me = dp[t][mask][v];
                if t == tim/2 { 
                    // hack = elephant means 2x the time as before but I have to start over at "AA" again.
                    v = *id.get("AA").unwrap();
                }

                dp[t + 1][mask][v] = max(dp[t + 1][mask][v], me);

                if spec_id.contains_key(&v) {
                    let vv = *spec_id.get(&v).unwrap();
                    if (mask >> vv & 1) == 0 {
                        let x = if t < tim/2 { tim/2 - t } else { tim - t};
                        dp[t + 1][mask | 1 << vv][v] = max(dp[t + 1][mask | 1 << vv][v], 
                            me + x as i32 * (*flow.get(&v).unwrap()));
                    }
                }

                for u in adj.get(&v).unwrap() {
                    let u = *id.get(u).unwrap();
                    dp[t + 1][mask][u] = max(dp[t + 1][mask][u], dp[t][mask][v]);
                }
            }
        }
    }

    let mut ans = 0;
    for mask in 0..((1<<n) as usize) {
        for v in 0..id.len() {
            ans = max(ans, dp[tim][mask][v]);
        }
    }
    println!("ans = {ans}");
}

#[macro_use] extern crate scan_fmt;

struct Monkey {
    id: usize,
    items: Vec<i64>,
    nxt: fn(i64) -> i64,
    test: i64,
    yes: usize,
    no: usize,
}

static LCM: i64 = 2*7*3*11*17*5*13*19;//hardcoded from input.


fn read_monkey() -> Monkey {
    let id = scanln_fmt!("Monkey {}:", usize).unwrap();
    println!("Reading Monkey {id}");

    let items = scanln_fmt!("Starting items: {}", String).unwrap();
    let items = items.split(",").map(|x| { x.parse::<i64>().unwrap() }).collect();

    fn build_nxt() -> fn(i64) -> i64 {
        
        let operation = scanln_fmt!("Operation: {}", String).unwrap();

        // a bit ugly, but okay
        match operation.as_str() {
            "new=old*3" => {
                fn tmp(old: i64) -> i64 { old*3 % LCM }
                return tmp
            }
            "new=old*11" => {
                fn tmp(old: i64) -> i64 { old*11 % LCM }
                return tmp
            }
            "new=old*19" => {
                fn tmp(old: i64) -> i64 { old*19 % LCM }
                return tmp
            }
            "new=old+6" => {
                fn tmp(old: i64) -> i64 { (old+6)%LCM }
                return tmp
            }
            "new=old+4" => {
                fn tmp(old: i64) -> i64 { (old+4)%LCM }
                return tmp
            }
            "new=old+8" => {
                fn tmp(old: i64) -> i64 { (old+8)%LCM }
                return tmp
            }
            "new=old+2" => {
                fn tmp(old: i64) -> i64 { (old+2)%LCM }
                return tmp
            }
            "new=old+3" => {
                fn tmp(old: i64) -> i64 { (old+3)%LCM }
                return tmp
            }
            "new=old*old" => {
                fn tmp(old: i64) -> i64 { old*old %LCM}
                return tmp
            }
            "new=old+5" => {
                fn tmp(old: i64) -> i64 { (old+5)%LCM }
                return tmp
            }
            _ => {
                assert!(false);
                fn foo(i: i64) -> i64 { i }
                return foo;
            }
        }
    }

    let nxt = build_nxt();

    let test = scanln_fmt!("Test: divisible by {}", i64).unwrap();
    let yes = scanln_fmt!("If true: throw to monkey {}", usize).unwrap();
    let no = scanln_fmt!("If false: throw to monkey {}", usize).unwrap();
    let monkey = Monkey{id, items, nxt, test, yes, no};
    monkey
}

fn solve() {
    const N: usize = 8;
    const R: usize = 10000;

    let mut monkeys = Vec::new();
    let mut all_items: Vec<(i64,usize)> = Vec::new();

    for _ in 0..N {
        monkeys.push(read_monkey());
    }

    for monkey in &monkeys {
        for x in &monkey.items {
            all_items.push((*x, monkey.id));
        }
    }

    let mut cnt: [i64; 8] = [0; N];

    for obj in all_items {
        let mut x = obj.0;
        let mut v = obj.1;
        for _ in 0..R {
            for i in 0..N {
                if v == i {
                    cnt[v] += 1;
                    let y = (monkeys[v].nxt)(x);
                    if y % monkeys[v].test == 0 {
                        v = monkeys[v].yes;
                    } else {
                        v = monkeys[v].no;
                    }
                    x = y;
                }
            }
        }
    }

    for x in cnt {
        println!("{x}");
    }
    cnt.sort();
    cnt.reverse();
    let ans = cnt[0] * cnt[1];
    println!("ans = {ans}");

}

fn main() {
    solve();
}
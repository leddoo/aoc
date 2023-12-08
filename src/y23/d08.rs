use std::collections::HashMap;


fn part_1(input: &str) -> u64 {
    let (instrs_str, table_str) = input.split_once("\n\n").unwrap();

    let mut table = HashMap::new();
    for line in table_str.lines() {
        let (k, v) = line.split_once(" = ").unwrap();

        let v = &v[1..];
        let v = &v[..v.len()-1];
        let (l, r) = v.split_once(", ").unwrap();

        let none = table.insert(k, [l, r]);
        assert!(none.is_none());
    }

    let mut n = 0;
    let mut at = "AAA";
    let mut instrs = instrs_str.bytes();
    while at != "ZZZ" {
        if instrs.len() == 0 {
            instrs = instrs_str.bytes();
        }
        let instr = instrs.next().unwrap();
        at = table[at][(instr == b'R') as usize];
        n += 1;
    }

    return n;
}


fn part_2(input: &str) -> u64 {
    let (instrs_str, table_str) = input.split_once("\n\n").unwrap();

    let mut ats = vec![];

    let mut table = HashMap::new();
    for line in table_str.lines() {
        let (k, v) = line.split_once(" = ").unwrap();

        let v = &v[1..];
        let v = &v[..v.len()-1];
        let (l, r) = v.split_once(", ").unwrap();

        let none = table.insert(k, [l, r]);
        assert!(none.is_none());

        if k.as_bytes()[2] == b'A' {
            ats.push(k);
        }
    }

    let mut steps = Vec::with_capacity(ats.len());
    for start in ats {
        let mut n = 0;
        let mut at = start;
        let mut instrs = instrs_str.bytes();

        macro_rules! step { () => {{
            if instrs.len() == 0 {
                instrs = instrs_str.bytes();
            }
            let instr = instrs.next().unwrap();
            at = table[at][(instr == b'R') as usize];
            at.as_bytes()[2] != b'Z'
        }}}

        while { n += 1; step!() } {}
        let first_stop = at;
        let first_n = n;
        steps.push(first_n);

        while { n += 1; step!() } {}
        assert_eq!(at, first_stop);
        assert_eq!(n,  2*first_n);
    }

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b)
        }
        return a;
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a*b / gcd(a, b)
    }

    let mut result = steps[0];
    for step in steps.iter().copied().skip(1) {
        result = lcm(result, step);
    }
    return result;
}


fn run(name: &str, f: impl FnOnce(&str) -> u64, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    run("part_1", part_1, include_str!("d08-test.txt"));
    run("part_1", part_1, include_str!("d08-prod.txt"));

    run("part_2", part_2, include_str!("d08-test-2.txt"));
    run("part_2", part_2, include_str!("d08-prod.txt"));
}


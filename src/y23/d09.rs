use itertools::Itertools;


fn part_1(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let mut seq = Vec::from_iter(
            line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap()));

        let mut lasts = vec![];
        while !seq.iter().all(|n| *n == 0) {
            lasts.push(*seq.last().unwrap());
            seq = Vec::from_iter(seq.iter().tuple_windows().map(|(a, b)| b - a));
        }

        result += lasts.iter().sum::<i32>() as i64;
    }

    return result as u64;
}


fn part_2(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let mut seq = Vec::from_iter(
            line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap()));

        let mut firsts = vec![];
        while !seq.iter().all(|n| *n == 0) {
            firsts.push(seq[0]);
            seq = Vec::from_iter(seq.iter().tuple_windows().map(|(a, b)| b - a));
        }

        let mut n = 0;
        for f in firsts.iter().copied().rev() {
            n = f - n;
        }
        result += n as i64;
    }

    return result as u64;
}


fn run(name: &str, f: impl FnOnce(&str) -> u64, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    println!("-- day 09 --");

    run("part_1", part_1, include_str!("d09-test.txt"));
    run("part_1", part_1, include_str!("d09-prod.txt"));

    run("part_2", part_2, include_str!("d09-test.txt"));
    run("part_2", part_2, include_str!("d09-prod.txt"));

    println!();
}


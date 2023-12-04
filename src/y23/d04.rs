use std::collections::VecDeque;


fn part_1(input: &str) -> i32 {
    input.lines().map(|line| {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, have) = line.split_once(" | ").unwrap();

        let mut table = [false; 256];
        for number in winning.split_whitespace() {
            let number = number.parse::<u8>().unwrap();
            table[number as usize] = true;
        }

        let mut num_matches = 0;
        for number in have.split_whitespace() {
            let number = number.parse::<u8>().unwrap();

            if table[number as usize] {
                num_matches += 1;
            }
        }

        if num_matches > 0 {
            1 << (num_matches - 1)
        }
        else { 0 }
    })
    .sum()
}


fn part_2(input: &str) -> i32 {
    let mut copies = VecDeque::new();
    copies.push_back(1);

    input.lines().map(|line| {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, have) = line.split_once(" | ").unwrap();

        let mut table = [false; 256];
        for number in winning.split_whitespace() {
            let number = number.parse::<u8>().unwrap();
            table[number as usize] = true;
        }

        let mut num_matches = 0;
        for number in have.split_whitespace() {
            let number = number.parse::<u8>().unwrap();

            if table[number as usize] {
                num_matches += 1;
            }
        }

        let n = copies.pop_front().unwrap();
        //println!("{num_matches} {n}");
        while copies.len() < num_matches {
            copies.push_back(1);
        }
        if copies.len() == 0 {
            copies.push_back(1);
        }

        for i in 0..num_matches {
            copies[i] += n;
        }

        return n;
    })
    .sum()
}


fn run(name: &str, f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    run("part_1", part_1, include_str!("d04-test.txt"));
    run("part_1", part_1, include_str!("d04-prod.txt"));

    run("part_2", part_2, include_str!("d04-test.txt"));
    run("part_2", part_2, include_str!("d04-prod.txt"));
}



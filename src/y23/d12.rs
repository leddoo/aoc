use std::collections::HashMap;


fn part_1(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let (row, pattern) = line.split_once(" ").unwrap();
        let pattern = Vec::from_iter(pattern.split(",").map(|n| n.parse::<u32>().unwrap()));

        fn rec(row: &[u8], pattern: &[u32]) -> u64 {
            if pattern.len() == 0 {
                return row.iter().all(|s| *s == b'.' || *s == b'?') as u64;
            }

            let n = pattern[0] as usize;
            let rest = &pattern[1..];

            let mut has_spring = false;
            let mut result = 0;
            for i in 0..row.len() {
                if i+n > row.len() { break }

                if has_spring { break }
                has_spring = row[i] == b'#';

                if !row[i..i+n].iter().all(|s| *s == b'#' || *s == b'?') {
                    continue;
                }

                if i+n == row.len() {
                    if rest.len() == 0 {
                        result += 1;
                    }
                }
                else if row[i+n] != b'#' {
                    result += rec(&row[i+n+1..], rest);
                }
            }
            return result;
        }

        result += rec(row.as_bytes(), &pattern);
    }
    return result;
}


fn part_2(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let (row, pattern) = line.split_once(" ").unwrap();
        let pattern = Vec::from_iter(pattern.split(",").map(|n| n.parse::<u32>().unwrap()));

        let row = {
            let mut result = String::new();
            result.push_str(row);
            result.push('?');
            result.push_str(row);
            result.push('?');
            result.push_str(row);
            result.push('?');
            result.push_str(row);
            result.push('?');
            result.push_str(row);
            result
        };
        let pattern = pattern.repeat(5);

        fn rec(row: &[u8], pattern: &[u32], row_i: usize, pattern_i: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
            if let Some(result) = memo.get(&(row_i, pattern_i)) {
                return *result;
            }

            let result = rec_core(row, pattern, row_i, pattern_i, memo);
            memo.insert((row_i, pattern_i), result);
            return result;
        }

        fn rec_core(row: &[u8], pattern: &[u32], row_i: usize, pattern_i: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
            if pattern.len() == 0 {
                return row.iter().all(|s| *s == b'.' || *s == b'?') as u64;
            }

            let n = pattern[0] as usize;
            let rest = &pattern[1..];

            let mut has_spring = false;
            let mut result = 0;
            for i in 0..row.len() {
                if i+n > row.len() { break }

                if has_spring { break }
                has_spring = row[i] == b'#';

                if !row[i..i+n].iter().all(|s| *s == b'#' || *s == b'?') {
                    continue;
                }

                if i+n == row.len() {
                    if rest.len() == 0 {
                        result += 1;
                    }
                }
                else if row[i+n] != b'#' {
                    result += rec(&row[i+n+1..], rest, row_i+i+n+1, pattern_i+1, memo);
                }
            }
            return result;
        }

        //println!("{row:?} {pattern:?} {}", rec(row.as_bytes(), &pattern, 0, 0, &mut HashMap::new()));

        result += rec(row.as_bytes(), &pattern, 0, 0, &mut HashMap::new());
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
    println!("-- day 12 --");

    run("part_1", part_1, include_str!("d12-test.txt"));
    run("part_1", part_1, include_str!("d12-prod.txt"));

    run("part_2", part_2, include_str!("d12-test.txt"));
    run("part_2", part_2, include_str!("d12-prod.txt"));

    println!();
}


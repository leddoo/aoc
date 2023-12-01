
fn part_1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for b in line.bytes() {
            if !b.is_ascii_digit() { continue }

            let b = b - b'0';

            if first.is_none() {
                first = Some(b);
            }
            else {
                last = Some(b);
            }
        }
        let a = first.unwrap();
        let b = last.unwrap_or(a);
        result += (a*10 + b) as i32;
    }
    return result;
}


fn part_2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        let mut bs = line.as_bytes();
        while let Some(b) = bs.get(0) {
            bs = &bs[1..];

            let b = match b {
                b'o' => {
                    if bs.starts_with(b"ne") {
                        1
                    }
                    else { continue }
                }

                b't' => {
                    if bs.starts_with(b"wo") {
                        2
                    }
                    else if bs.starts_with(b"hree") {
                        3
                    }
                    else { continue }
                }

                b'f' => {
                    if bs.starts_with(b"our") {
                        4
                    }
                    else if bs.starts_with(b"ive") {
                        5
                    }
                    else { continue }
                }

                b's' => {
                    if bs.starts_with(b"ix") {
                        6
                    }
                    else if bs.starts_with(b"even") {
                        7
                    }
                    else { continue }
                }

                b'e' => {
                    if bs.starts_with(b"ight") {
                        8
                    }
                    else { continue }
                }

                b'n' => {
                    if bs.starts_with(b"ine") {
                        9
                    }
                    else { continue }
                }

                b'0'..=b'9' => {
                    b - b'0'
                }

                _ => continue,
            };

            if first.is_none() {
                first = Some(b);
            }
            else {
                last = Some(b);
            }
        }
        let a = first.unwrap();
        let b = last.unwrap_or(a);
        //println!("{line:?} {a}{b}");
        result += (a*10 + b) as i32;
    }
    return result;
}


fn run(f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("result: {result} in {dt:?}");
}

pub fn main() {
    run(part_1, include_str!("d01-test.txt"));
    run(part_1, include_str!("d01-prod.txt"));
    run(part_2, include_str!("d01-test-2.txt"));
    run(part_2, include_str!("d01-prod.txt"));
}



fn part_1(input: &str) -> i32 {
    let w = input.bytes().position(|b| b == b'\n').unwrap();
    let s = w+1;
    let h = input.len()/s;

    let mut result = 0;

    let input = input.as_bytes();
    for y in 0..h {
        let mut before = false;

        let line = &input[y*s .. y*s+w];
        assert_eq!(line.len(), w);

        let prev =
            if y > 0 { &input[(y-1)*s .. (y-1)*s+w] }
            else     { line };

        let next =
            if y+1 < h { &input[(y+1)*s .. (y+1)*s+w] }
            else     { line };

        let mut x = 0;
        while x < w {
            match line[x] {
                b'.' => {
                    before = false;
                    x += 1;
                }

                b'0'..=b'9' => {
                    let x0 = x;
                    x += 1;
                    while x < w && line[x].is_ascii_digit() {
                        x += 1;
                    }

                    let scan_x0 = if x0 > 0 { x0 - 1 } else { x0 };
                    let scan_x1 = if x  < w { x  + 1 } else { x };

                    let mut ok = before;
                    if !ok {
                        for sx in scan_x0..scan_x1 {
                            if !(prev[sx].is_ascii_digit() || prev[sx] == b'.')
                            || !(next[sx].is_ascii_digit() || next[sx] == b'.') {
                                ok = true;
                                break;
                            }
                        }
                    }
                    if !ok && x < w {
                        if !(line[x].is_ascii_digit() || line[x] == b'.') {
                            ok = true;
                        }
                    }

                    if ok {
                        let nonsense = unsafe { core::str::from_utf8_unchecked(line) };
                        result += nonsense[x0..x].parse::<i32>().unwrap();
                    }
                }

                _ => {
                    before = true;
                    x += 1;
                }
            }
        }
    }

    return result;
}


fn part_2(input: &str) -> i32 {
    let w = input.bytes().position(|b| b == b'\n').unwrap();
    let s = w+1;
    let h = input.len()/s;

    let mut result = 0;

    let input = input.as_bytes();
    for y in 0..h {
        let line = &input[y*s .. y*s+w];
        assert_eq!(line.len(), w);

        let prev =
            if y > 0 { &input[(y-1)*s .. (y-1)*s+w] }
            else     { line };

        let next =
            if y+1 < h { &input[(y+1)*s .. (y+1)*s+w] }
            else     { line };

        for x in 0..w {
            if line[x] != b'*' {
                continue;
            }

            let tm = prev[x].is_ascii_digit() as i32;
            let bm = next[x].is_ascii_digit() as i32;
            let tl = if x   > 0 { prev[x-1].is_ascii_digit() as i32 & !tm } else { 0 };
            let ml = if x   > 0 { line[x-1].is_ascii_digit() as i32       } else { 0 };
            let bl = if x   > 0 { next[x-1].is_ascii_digit() as i32 & !bm } else { 0 };
            let tr = if x+1 < w { prev[x+1].is_ascii_digit() as i32 & !tm } else { 0 };
            let mr = if x+1 < w { line[x+1].is_ascii_digit() as i32       } else { 0 };
            let br = if x+1 < w { next[x+1].is_ascii_digit() as i32 & !bm } else { 0 };

            let num_numbers =
                  tl + tm + tr
                + ml      + mr
                + bl + bm + br;

            if num_numbers != 2 {
                continue;
            }

            let parse = |line: &[u8], pos: usize| -> i32 {
                let mut x0 = pos;
                while x0 > 0 && line[x0 - 1].is_ascii_digit() {
                    x0 -= 1;
                }

                let mut x1 = pos;
                while x1 < line.len() && line[x1].is_ascii_digit() {
                    x1 += 1;
                }

                let nonsense = unsafe { core::str::from_utf8_unchecked(line) };
                nonsense[x0..x1].parse().unwrap()
            };

            let mut ratio = 1;
            if tl != 0 { ratio *= parse(prev, x-1); }
            if tm != 0 { ratio *= parse(prev, x  ); }
            if tr != 0 { ratio *= parse(prev, x+1); }
            if ml != 0 { ratio *= parse(line, x-1); }
            if mr != 0 { ratio *= parse(line, x+1); }
            if bl != 0 { ratio *= parse(next, x-1); }
            if bm != 0 { ratio *= parse(next, x  ); }
            if br != 0 { ratio *= parse(next, x+1); }
            result += ratio;
        }
    }

    return result;
}


fn run(name: &str, f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    run("part_1", part_1, include_str!("d03-test.txt"));
    run("part_1", part_1, include_str!("d03-prod.txt"));

    run("part_2", part_2, include_str!("d03-test.txt"));
    run("part_2", part_2, include_str!("d03-prod.txt"));
}


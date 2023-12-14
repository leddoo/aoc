use std::collections::HashMap;


struct Grid<'a> {
    data: &'a [u8],
    w: usize,
    s: usize,
    h: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let data = input.as_bytes();

        let w = data.iter().position(|b| *b == b'\n').unwrap();
        let s = w+1;
        let h = input.len() / s;
        assert!(h*s == input.len());

        return Self { data, w, s, h };
    }
}

impl<'a> core::ops::Index<(usize, usize)> for Grid<'a> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y*self.s + x]
    }
}


fn part_1(input: &str) -> u64 {
    let g = Grid::new(input);

    let mut result = 0;
    for x in 0..g.w {
        let mut load = 0;
        let mut chain = 0;
        for y in (0..g.h).rev() {
            if g[(x, y)] == b'#' {
                for i in 0..chain { result += load - i; }
                chain = 0;
            }
            if g[(x, y)] == b'O' {
                chain += 1;
            }
            load += 1;
        }
        for i in 0..chain { result += load - i; }
    }
    return result;
}


fn part_2(input: &str) -> u64 {
    let g = Grid::new(input);

    let mut buf = input.as_bytes().to_vec();

    fn cycle(buf: &mut [u8], w: usize, s: usize, h: usize) {
        // north.
        for x in 0..w {
            let mut y_stop = 0;
            for y in 0..h {
                let at = buf[s*y + x];
                if at == b'O' {
                    if y_stop != y {
                        buf[s*y_stop + x] = b'O';
                        buf[s*y      + x] = b'.';
                    }
                    y_stop += 1;
                }
                if at == b'#' {
                    y_stop = y + 1;
                }
            }
        }

        // west.
        for y in 0..h {
            let mut x_stop = 0;
            for x in 0..w {
                let at = buf[s*y + x];
                if at == b'O' {
                    if x_stop != x {
                        buf[s*y + x_stop] = b'O';
                        buf[s*y + x     ] = b'.';
                    }
                    x_stop += 1;
                }
                if at == b'#' {
                    x_stop = x + 1;
                }
            }
        }

        // south.
        for x in 0..w {
            let mut y_stop = h-1;
            for y in (0..h).rev() {
                let at = buf[s*y + x];
                if at == b'O' {
                    if y_stop != y {
                        buf[s*y_stop + x] = b'O';
                        buf[s*y      + x] = b'.';
                    }
                    y_stop = y_stop.wrapping_sub(1);
                }
                if at == b'#' {
                    y_stop = y.wrapping_sub(1);
                }
            }
        }

        // east.
        for y in 0..h {
            let mut x_stop = w-1;
            for x in (0..w).rev() {
                let at = buf[s*y + x];
                if at == b'O' {
                    if x_stop != x {
                        buf[s*y + x_stop] = b'O';
                        buf[s*y + x     ] = b'.';
                    }
                    x_stop = x_stop.wrapping_sub(1);
                }
                if at == b'#' {
                    x_stop = x.wrapping_sub(1);
                }
            }
        }

        //for y in 0..h { println!("{}", core::str::from_utf8(&buf[y*s..y*s+w]).unwrap()) } println!()
    }

    let mut n = 0u64;
    let mut visited = HashMap::new();
    loop {
        cycle(&mut buf, g.w, g.s, g.h);
        n += 1;

        if let Some(old_n) = visited.get(&buf) {
            let mut remaining = 1_000_000_000;
            remaining -= n;
            remaining %= n - old_n;

            for _ in 0..remaining {
                cycle(&mut buf, g.w, g.s, g.h);
            }

            return (0..g.h).map(|y| {
                ((0..g.w).filter(|x| buf[y*g.s + x] == b'O').count() * (g.h - y)) as u64
            }).sum();
        }
        else {
            visited.insert(buf.clone(), n);
        }
    }
}


fn run(name: &str, f: impl FnOnce(&str) -> u64, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    println!("-- day 14 --");

    run("part_1", part_1, include_str!("d14-test.txt"));
    run("part_1", part_1, include_str!("d14-prod.txt"));

    run("part_2", part_2, include_str!("d14-test.txt"));
    run("part_2", part_2, include_str!("d14-prod.txt"));

    println!();
}



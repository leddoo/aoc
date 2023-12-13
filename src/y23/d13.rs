
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
        let h = (input.len() + s-1) / s;
        assert!(h*s == input.len() || h*s == input.len() + 1);

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
    let mut result = 0;
    for grid in input.split("\n\n") {
        let g = Grid::new(grid);

        let mut mirror = None;
        for c in 1..g.h {
            let first = c.checked_sub(g.h-c).unwrap_or(0);
            mirror = Some(c);
            for i in first..c {
                let y0 = i;
                let y1 = c + (c - i - 1);
                for x in 0..g.w {
                    if g[(x, y0)] != g[(x, y1)] {
                        mirror = None;
                        break;
                    }
                }
            }
            if mirror.is_some() { break }
        }
        if let Some(rs) = mirror {
            result += 100*rs as u64;
            continue;
        }

        let mut mirror = None;
        for c in 1..g.w {
            let first = c.checked_sub(g.w-c).unwrap_or(0);
            mirror = Some(c);
            for i in first..c {
                let x0 = i;
                let x1 = c + (c - i - 1);
                for y in 0..g.h {
                    if g[(x0, y)] != g[(x1, y)] {
                        mirror = None;
                        break;
                    }
                }
            }
            if mirror.is_some() { break }
        }
        if let Some(cs) = mirror {
            result += cs as u64;
            continue;
        }
    }
    return result;
}


fn part_2(input: &str) -> u64 {
    let mut result = 0;
    for grid in input.split("\n\n") {
        let g = Grid::new(grid);

        let mut mirror = None;
        for c in 1..g.h {
            let first = c.checked_sub(g.h-c).unwrap_or(0);
            let mut errors = 0;
            for i in first..c {
                let y0 = i;
                let y1 = c + (c - i - 1);
                for x in 0..g.w {
                    if g[(x, y0)] != g[(x, y1)] {
                        errors += 1;
                        if errors != 1 { break }
                    }
                }
            }
            if errors == 1 { mirror = Some(c); break }
        }
        if let Some(rs) = mirror {
            result += 100*rs as u64;
            continue;
        }

        let mut mirror = None;
        for c in 1..g.w {
            let first = c.checked_sub(g.w-c).unwrap_or(0);
            let mut errors = 0;
            for i in first..c {
                let x0 = i;
                let x1 = c + (c - i - 1);
                for y in 0..g.h {
                    if g[(x0, y)] != g[(x1, y)] {
                        errors += 1;
                        if errors != 1 { break }
                    }
                }
            }
            if errors == 1 { mirror = Some(c); break }
        }
        if let Some(cs) = mirror {
            result += cs as u64;
            continue;
        }
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
    println!("-- day 13 --");

    run("part_1", part_1, include_str!("d13-test.txt"));
    run("part_1", part_1, include_str!("d13-prod.txt"));

    run("part_2", part_2, include_str!("d13-test.txt"));
    run("part_2", part_2, include_str!("d13-prod.txt"));

    println!();
}


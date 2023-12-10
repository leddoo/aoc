#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Up, Left, Down, Right
}

impl Step {
    fn apply(self, x: &mut usize, y: &mut usize) {
        match self {
            Step::Up    => *y -= 1,
            Step::Left  => *x -= 1,
            Step::Down  => *y += 1,
            Step::Right => *x += 1,
        }
    }
}


struct Grid<'a> {
    data: &'a [u8],
    w: usize,
    s: usize,
    h: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> (Self, (usize, usize)) {
        let data = input.as_bytes();

        let w = data.iter().position(|b| *b == b'\n').unwrap();
        let s = w+1;
        let h = input.len() / s;
        assert_eq!(h*s, input.len());

        let this = Self { data, w, s, h };

        let start_idx = data.iter().position(|b| *b == b'S').unwrap();
        let start_x = start_idx % s;
        let start_y = start_idx / s;

        return (this, (start_x, start_y));
    }

    #[inline]
    fn check_step(&self, x: usize, y: usize, step: Step) -> bool {
        match step {
            Step::Up => {
                if y == 0 { return false }

                match self.data[y*self.s + x] {
                    b'|' => (),
                    b'-' => return false,
                    b'L' => (),
                    b'J' => (),
                    b'7' => return false,
                    b'F' => return false,
                    _ => (),
                }

                match self.data[(y-1)*self.s + x] {
                    b'|' => true,
                    b'-' => false,
                    b'L' => false,
                    b'J' => false,
                    b'7' => true,
                    b'F' => true,
                    _ => false,
                }
            }

            Step::Left => {
                if x == 0 { return false }

                match self.data[y*self.s + x] {
                    b'|' => return false,
                    b'-' => (),
                    b'L' => return false,
                    b'J' => (),
                    b'7' => (),
                    b'F' => return false,
                    _ => (),
                }

                match self.data[y*self.s + x-1] {
                    b'|' => false,
                    b'-' => true,
                    b'L' => true,
                    b'J' => false,
                    b'7' => false,
                    b'F' => true,
                    _ => false,
                }
            }

            Step::Down => {
                if y == self.h-1 { return false }

                match self.data[y*self.s + x] {
                    b'|' => (),
                    b'-' => return false,
                    b'L' => return false,
                    b'J' => return false,
                    b'7' => (),
                    b'F' => (),
                    _ => (),
                }

                match self.data[(y+1)*self.s + x] {
                    b'|' => true,
                    b'-' => false,
                    b'L' => true,
                    b'J' => true,
                    b'7' => false,
                    b'F' => false,
                    _ => false,
                }
            }

            Step::Right => {
                if x == self.w-1 { return false }

                match self.data[y*self.s + x] {
                    b'|' => return false,
                    b'-' => (),
                    b'L' => (),
                    b'J' => return false,
                    b'7' => return false,
                    b'F' => (),
                    _ => (),
                }

                match self.data[y*self.s + x+1] {
                    b'|' => false,
                    b'-' => true,
                    b'L' => false,
                    b'J' => true,
                    b'7' => true,
                    b'F' => false,
                    _ => false,
                }
            }
        }
    }
}

fn part_1(input: &str) -> u64 {
    let (grid, (start_x, start_y)) = Grid::new(input);

    for (mut prev, first) in [(Step::Down, Step::Up), (Step::Right, Step::Left), (Step::Up, Step::Down), (Step::Left, Step::Right)] {
        if !grid.check_step(start_x, start_y, first) { continue }

        let mut x = start_x;
        let mut y = start_y;

        let mut len = 1;
        first.apply(&mut x, &mut y);

        let mut stepped = true;
        while stepped {
            stepped = false;

            for (next_prev, next) in [(Step::Down, Step::Up), (Step::Right, Step::Left), (Step::Up, Step::Down), (Step::Left, Step::Right)] {
                if next == prev { continue }

                if grid.check_step(x, y, next) {
                    stepped = true;
                    len += 1;
                    next.apply(&mut x, &mut y);
                    prev = next_prev;
                    break;
                }
            }
        }

        assert!((x as isize - start_x as isize).abs()
            +   (y as isize - start_y as isize).abs()
            == 1);

        return (len+1) / 2;
    }
    unreachable!()
}


fn part_2(input: &str) -> u64 {
    let (grid, (start_x, start_y)) = Grid::new(input);

    let mut windings = vec![0i8; grid.s*grid.h];

    for (mut prev, first) in [(Step::Down, Step::Up), (Step::Right, Step::Left), (Step::Up, Step::Down), (Step::Left, Step::Right)] {
        if !grid.check_step(start_x, start_y, first) { continue }

        let mut x = start_x;
        let mut y = start_y;

        if first == Step::Up   { windings[y*grid.s + x] -= 1 }
        if first == Step::Down { windings[y*grid.s + x] += 1 }
        first.apply(&mut x, &mut y);
        if first == Step::Up   { windings[y*grid.s + x] -= 1 }
        if first == Step::Down { windings[y*grid.s + x] += 1 }

        let mut stepped = true;
        while stepped {
            stepped = false;

            for (next_prev, next) in [(Step::Down, Step::Up), (Step::Right, Step::Left), (Step::Up, Step::Down), (Step::Left, Step::Right)] {
                if next == prev { continue }

                if grid.check_step(x, y, next) {
                    stepped = true;

                    if next == Step::Up   { windings[y*grid.s + x] -= 1 }
                    if next == Step::Down { windings[y*grid.s + x] += 1 }
                    next.apply(&mut x, &mut y);
                    if next == Step::Up   { windings[y*grid.s + x] -= 1 }
                    if next == Step::Down { windings[y*grid.s + x] += 1 }

                    prev = next_prev;
                    break;
                }
            }
        }

        assert!((x as isize - start_x as isize).abs()
            +   (y as isize - start_y as isize).abs()
            == 1);

        if y > start_y {
            windings[start_y*grid.s + start_x] -= 1;
            windings[y*grid.s + x] -= 1;
        }
        if y < start_y {
            windings[start_y*grid.s + start_x] += 1;
            windings[y*grid.s + x] += 1;
        }


        let mut inside = 0;
        for y in 0..grid.h {
            let mut was_inside = false;
            let mut w = 0i8;
            for x in 0..grid.w {
                w += windings[y*grid.s + x];

                let is_inside = w.abs() >= 2 && w % 2 == 0;
                inside += (is_inside & was_inside) as u64;

                was_inside = is_inside;
            }
            assert_eq!(w, 0);
        }
        return inside;
    }
    unreachable!()
}


fn run(name: &str, f: impl FnOnce(&str) -> u64, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    println!("-- day 10 --");

    run("part_1", part_1, include_str!("d10-test.txt"));
    run("part_1", part_1, include_str!("d10-prod.txt"));

    run("part_2", part_2, include_str!("d10-test.txt"));
    run("part_2", part_2, include_str!("d10-test-2.txt"));
    run("part_2", part_2, include_str!("d10-test-3.txt"));
    run("part_2", part_2, include_str!("d10-prod.txt"));

    println!();
}



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
        assert_eq!(h*s, input.len());

        return Self { data, w, s, h };
    }
}


fn solution(input: &str, grow: i64) -> u64 {
    let g = Grid::new(input);

    let mut galaxies = vec![];
    let mut has_galaxy_x = vec![false; g.w];
    let mut has_galaxy_y = vec![false; g.h];
    for y in 0..g.h {
        for x in 0..g.w {
            if g.data[y*g.s + x] == b'#' {
                galaxies.push((x, y));
                has_galaxy_x[x] = true;
                has_galaxy_y[y] = true;
            }
        }
    }

    let num_gaps_x_acc = {
        let mut n = 0;
        Vec::from_iter(has_galaxy_x.iter()
           .map(|x| { if !x { n += 1 }; n }))
    };

    let num_gaps_y_acc = {
        let mut n = 0;
        Vec::from_iter(has_galaxy_y.iter()
           .map(|x| { if !x { n += 1 }; n }))
    };

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];

            let sx = num_gaps_x_acc[x1.max(x2)] - num_gaps_x_acc[x1.min(x2)];
            let sy = num_gaps_y_acc[y1.max(y2)] - num_gaps_y_acc[y1.min(y2)];

            let dx = (x2 as i64 - x1 as i64).abs() + sx*grow;
            let dy = (y2 as i64 - y1 as i64).abs() + sy*grow;

            //println!("{}, {}: {dx} {dy} {} {sx} {sy}", i+1, j+1, dx+dy);

            result += dx + dy;
        }
    }

    return result as u64;
}


fn part_1(input: &str) -> u64 {
    solution(input, 2-1)
}


fn part_2(input: &str) -> u64 {
    solution(input, 1_000_000-1)
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

    run("part_1", part_1, include_str!("d11-test.txt"));
    run("part_1", part_1, include_str!("d11-prod.txt"));

    run("part_2", part_2, include_str!("d11-test.txt"));
    run("part_2", part_2, include_str!("d11-prod.txt"));

    println!();
}


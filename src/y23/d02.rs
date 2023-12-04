
fn part_1(input: &str) -> i32 {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    input.lines()
    .map(|line| {
        let (game, sets) = line.split_once(": ").unwrap();
        let ok = sets.split("; ").all(|set| {
            set.split(", ").all(|sample| {
                let (count, color) = sample.split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();
                match color {
                    "red"   => count <= red_cubes,
                    "green" => count <= green_cubes,
                    "blue"  => count <= blue_cubes,
                    _ => unreachable!()
                }
            })
        });

        if ok {
            let (_, id) = game.split_once(" ").unwrap();
            id.parse().unwrap()
        }
        else { 0 }
    })
    .sum()
}


fn part_2(input: &str) -> i32 {
    input.lines()
    .map(|line| {
        let (_, sets) = line.split_once(": ").unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        sets.split("; ").for_each(|set| {
            set.split(", ").for_each(|sample| {
                let (count, color) = sample.split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();
                match color {
                    "red"   => red   = red.max(count),
                    "green" => green = green.max(count),
                    "blue"  => blue  = blue.max(count),
                    _ => unreachable!()
                }
            })
        });

        red*green*blue
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
    run("part_1", part_1, include_str!("d02-test.txt"));
    run("part_1", part_1, include_str!("d02-prod.txt"));

    run("part_2", part_2, include_str!("d02-test.txt"));
    run("part_2", part_2, include_str!("d02-prod.txt"));
}


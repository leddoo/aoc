
fn part_1(input: &str) -> i32 {
    /*
        dst = (tmax - t)*t
            = tmax*t - t*t

        0 = t*t - tmax*t + dst

        t01 = ceil(tmax/2 +- sqrt(tmax^2/4 - dst))
    */

    let mut lines = input.lines();
    let ts = Vec::from_iter(
        lines.next().unwrap()
        .split_once(":").unwrap().1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap()));

    let ds = Vec::from_iter(
        lines.next().unwrap()
        .split_once(":").unwrap().1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap()));


    let mut result = 1;
    for (t, d) in ts.iter().copied().zip(ds.iter().copied()) {
        let t = t as f32;
        let d = d as f32 + 1.0;

        let a = t/2.0;
        let b = (t*t/4.0 - d).sqrt();
        let t0 = (a - b).ceil() as i32;
        let t1 = (a + b).floor() as i32;

        result *= t1 - t0 + 1;
    }
    return result;
}


fn part_2(input: &str) -> i32 {
    let mut lines = input.lines();

    let t = String::from_iter(
        lines.next().unwrap()
        .split_once(":").unwrap().1
        .trim()
        .split_whitespace()
    ).parse::<i64>().unwrap();

    let d = String::from_iter(
        lines.next().unwrap()
        .split_once(":").unwrap().1
        .trim()
        .split_whitespace()
    ).parse::<i64>().unwrap();

    let t = t as f64;
    let d = d as f64 + 1.0;

    let a = t/2.0;
    let b = (t*t/4.0 - d).sqrt();
    let t0 = (a - b).ceil() as i64;
    let t1 = (a + b).floor() as i64;

    return (t1 - t0 + 1) as i32;
}


fn run(name: &str, f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    run("part_1", part_1, include_str!("d06-test.txt"));
    run("part_1", part_1, include_str!("d06-prod.txt"));

    run("part_2", part_2, include_str!("d06-test.txt"));
    run("part_2", part_2, include_str!("d06-prod.txt"));
}


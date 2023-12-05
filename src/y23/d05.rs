
fn part_1(input: &str) -> i32 {
    let mut parts = input.split("\n\n");
    let seeds = parts.next().unwrap();

    let mut seeds = Vec::from_iter(
        seeds.split_once(": ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap()));

    let mut mapped = vec![false; seeds.len()];

    for map in parts {
        let mut lines = map.lines();
        lines.next();

        for x in &mut mapped { *x = false }

        for line in lines {
            let mut numbers = line.split_whitespace();
            let dst = numbers.next().unwrap().parse::<u32>().unwrap();
            let src = numbers.next().unwrap().parse::<u32>().unwrap();
            let len = numbers.next().unwrap().parse::<u32>().unwrap();
            assert!(numbers.next().is_none());

            for (i, seed) in seeds.iter_mut().enumerate() {
                if !mapped[i] && *seed >= src && *seed - src < len {
                    mapped[i] = true;
                    *seed = dst + (*seed - src);
                }
            }
        }
    }

    *seeds.iter().min().unwrap() as i32
}


fn part_2(input: &str) -> i32 {
    let mut parts = input.split("\n\n");
    let seeds = parts.next().unwrap();

    let seeds = Vec::from_iter(
        seeds.split_once(": ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap()));
    assert!(seeds.len() % 2 == 0);

    let mut seeds = Vec::from_iter((0..seeds.len()/2).map(|i| (seeds[2*i], seeds[2*i+1])));

    let mut mapped = vec![false; seeds.len()];

    for map in parts {
        let mut lines = map.lines();
        lines.next();

        assert_eq!(seeds.len(), mapped.len());
        for x in &mut mapped { *x = false }

        for line in lines {
            let mut numbers = line.split_whitespace();
            let dst = numbers.next().unwrap().parse::<u64>().unwrap();
            let src = numbers.next().unwrap().parse::<u64>().unwrap();
            let len = numbers.next().unwrap().parse::<u64>().unwrap();
            assert!(numbers.next().is_none());

            for i in 0..seeds.len() {
                if mapped[i] { continue }

                let (mut begin, mut range_len) = seeds[i];

                if begin + range_len <= src
                || src + len <= begin {
                    continue;
                }

                // left
                if begin < src {
                    let new_len = src - begin;
                    mapped.push(false);
                    seeds.push((begin, new_len));
                    range_len -= new_len;
                    begin = src;
                }

                // right
                if begin + range_len > src + len {
                    let new_len = begin + range_len - (src + len);
                    mapped.push(false);
                    seeds.push((src + len, new_len));
                    range_len -= new_len;
                }

                // middle
                {
                    mapped[i] = true;
                    seeds[i] = (dst + (begin - src), range_len);
                }
            }
        }
    }

    seeds.iter().copied()
    .map(|(b, l)| if l > 0 { b } else { u64::MAX })
    .min().unwrap() as i32
}


fn run(name: &str, f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    run("part_1", part_1, include_str!("d05-test.txt"));
    run("part_1", part_1, include_str!("d05-prod.txt"));

    run("part_2", part_2, include_str!("d05-test.txt"));
    run("part_2", part_2, include_str!("d05-prod.txt"));
}


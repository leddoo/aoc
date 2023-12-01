use std::collections::HashSet;


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
                    if bs.starts_with(b"ne") { 1 }
                    else { continue }
                }

                b't' => {
                    if bs.starts_with(b"wo") { 2 }
                    else if bs.starts_with(b"hree") { 3 }
                    else { continue }
                }

                b'f' => {
                    if bs.starts_with(b"our") { 4 }
                    else if bs.starts_with(b"ive") { 5 }
                    else { continue }
                }

                b's' => {
                    if bs.starts_with(b"ix") { 6 }
                    else if bs.starts_with(b"even") { 7 }
                    else { continue }
                }

                b'e' => {
                    if bs.starts_with(b"ight") { 8 }
                    else { continue }
                }

                b'n' => {
                    if bs.starts_with(b"ine") { 9 }
                    else { continue }
                }

                b'0'..=b'9' => b - b'0',

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


fn part_2_make_fsm() -> Vec<(u8, u8)> {
    use std::collections::HashMap;

    let words = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    let mut vocab = HashSet::new();
    for word in words {
        for c in word.as_bytes() {
            vocab.insert(*c);
        }
    }

    let mut word_map = HashMap::new();
    for (i, word) in words.iter().enumerate() {
        word_map.insert(word.as_bytes(), (i + 1) as u8);
    }

    let mut states = vec![""];
    let mut state_map = HashMap::new();
    state_map.insert("", 0);

    let mut n = 0;
    loop {
        n += 1;

        let mut any = false;
        for word in words {
            if n < word.len() {
                any = true;

                let buf = &word[..n];
                if !state_map.contains_key(buf) {
                    state_map.insert(buf, states.len() as u8);
                    states.push(buf);
                }
            }
        }
        if !any { break }
    }

    let mut table = vec![(0u8, 0u8); states.len()*256];
    for (i, state) in states.iter().enumerate() {
        let map = &mut table[i*256 .. (i+1)*256];

        map[b'1' as usize] = (0, 1);
        map[b'2' as usize] = (0, 2);
        map[b'3' as usize] = (0, 3);
        map[b'4' as usize] = (0, 4);
        map[b'5' as usize] = (0, 5);
        map[b'6' as usize] = (0, 6);
        map[b'7' as usize] = (0, 7);
        map[b'8' as usize] = (0, 8);
        map[b'9' as usize] = (0, 9);

        map[b'\n' as usize] = (0, 255);

        let mut buf = Vec::from(state.as_bytes());

        for c in vocab.iter().copied() {
            buf.push(c);

            let a = word_map.get(&*buf).copied().unwrap_or(0);

            //println!("{:?}: {a}", core::str::from_utf8(&buf).unwrap());

            let mut s = 0;
            for (i, state) in states.iter().enumerate() {
                if buf.ends_with(state.as_bytes()) {
                    //println!("match {:?}", state);
                    if s != 0 {
                        let old_match = states[s];
                        if state.ends_with(old_match) {
                            s = i;
                        }
                        else { assert!(old_match.ends_with(state)) }
                    }
                    else {
                        s = i;
                    }
                }
            }

            map[c as usize] = (s as u8, a);

            buf.pop();
        }

        //println!("{map:?}");
    }

    return table;
}

fn part_2_fsm(input: &str, table: &[(u8, u8)]) -> i32 {
    let mut result = 0;
    let mut n = 0;
    let mut vs = [0, 0];

    let mut state = 0;

    for i in input.bytes() {
        let (s, a) = unsafe {
            *table.get_unchecked(state as usize * 256 + i as usize)
        };
        state = s;

        //println!("{state} {:?} -> {s} {a}", i as char);

        if a == 255 {
            result += match n {
                0 => unreachable!(),
                1 => vs[0]*10 + vs[0],
                _ => vs[0]*10 + vs[1],
            };

            n = 0;
        }
        else if a != 0 {
            //println!("a: {a}");
            vs[n.min(1)] = a as i32;
            n += 1;
        }
    }
    assert_eq!(n, 0);

    return result;
}


fn run(f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("result: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    for _ in 0..1000 {
        if 0==1 { break }
        part_1(include_str!("d01-prod.txt"));
        part_2(include_str!("d01-prod.txt"));
    }

    println!("part 1");
    run(part_1, include_str!("d01-test.txt"));
    run(part_1, include_str!("d01-prod.txt"));
    println!();

    println!("part 2");
    run(part_2, include_str!("d01-test-2.txt"));
    run(part_2, include_str!("d01-prod.txt"));
    println!();

    println!("part 2 fsm");
    let fsm = part_2_make_fsm();
    run(|i| part_2_fsm(i, &fsm), include_str!("d01-test-2.txt"));
    run(|i| part_2_fsm(i, &fsm), include_str!("d01-prod.txt"));
    println!();
}


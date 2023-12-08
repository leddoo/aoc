use std::collections::{HashSet, HashMap};


use rayon::prelude::*;


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


fn part_2_make_fsm_ex(rev: bool, eol: bool) -> Vec<(u8, u8)> {
    let words =
        if !rev { [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ] }
        else    { [ "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin" ] };

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

        if eol {
            map[b'\n' as usize] = (0, 255);
        }

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

fn part_2_make_fsm() -> Vec<(u8, u8)> {
    part_2_make_fsm_ex(false, true)
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

fn part_2_fsm_threaded(input: &str, table: &[(u8, u8)]) -> i32 {
    let n = if input.len() < 128 { 1 } else { 12 };

    (0..n)
    .into_par_iter()
    .map(move |i| {
        let chunk_size = input.len() / n;

        let mut begin = chunk_size * i;
        let mut end = chunk_size * (i+1);

        while begin > 0 && input.as_bytes()[begin - 1] != b'\n' {
            begin -= 1;
        }

        if i < n-1 {
            while end > begin && input.as_bytes()[end - 1] != b'\n' {
                end -= 1;
            }
        }
        else {
            end = input.len();
        }

        //println!("{begin}..{end}");

        part_2_fsm(&input[begin..end], table)
    }).sum()
}


fn part_2_make_fsm2() -> (Vec<(u8, u8)>, Vec<(u8, u8)>) {
    (part_2_make_fsm_ex(false, false),
     part_2_make_fsm_ex(true,  false))
}

fn part_2_fsm2(input: &str, fwd: &[(u8, u8)], bwd: &[(u8, u8)]) -> i32 {
    let mut result = 0;

    for line in input.as_bytes().split(|at| *at == b'\n') {
        let mut vs = [0, 0];
        let mut state = 0;

        for i in line.iter().copied() {
            let (s, a) = unsafe {
                *fwd.get_unchecked(state as usize * 256 + i as usize)
            };
            state = s;

            //println!("{state} {:?} -> {s} {a}", i as char);

            if a != 0 {
                //println!("a: {a}");
                vs[0] = a as i32;
                break;
            }
        }

        for i in line.iter().rev().copied() {
            let (s, a) = unsafe {
                *bwd.get_unchecked(state as usize * 256 + i as usize)
            };
            state = s;

            //println!("{state} {:?} -> {s} {a}", i as char);

            if a != 0 {
                //println!("a: {a}");
                vs[1] = a as i32;
                break;
            }
        }

        result += vs[0]*10 + vs[1];
    }

    return result;
}

fn part_2_fsm2_threaded(input: &str, fwd: &[(u8, u8)], bwd: &[(u8, u8)]) -> i32 {
    let n = if input.len() < 128 { 1 } else { 12 };

    (0..n)
    .into_par_iter()
    .map(move |i| {
        let chunk_size = input.len() / n;

        let mut begin = chunk_size * i;
        let mut end = chunk_size * (i+1);

        while begin > 0 && input.as_bytes()[begin - 1] != b'\n' {
            begin -= 1;
        }

        if i < n-1 {
            while end > begin && input.as_bytes()[end - 1] != b'\n' {
                end -= 1;
            }
        }
        else {
            end = input.len();
        }

        //println!("{begin}..{end}");

        part_2_fsm2(&input[begin..end], fwd, bwd)
    }).sum()
}


fn part_2_fsm2_vect(input: &str, fwd: &[(u8, u8)], bwd: &[(u8, u8)]) -> i32 {
    let mut result = 0;

    let mut input = input.as_bytes();
    while input.len() > 0 {
        let line = {
            let mut len = 0;

            while len + 8 < input.len() {
                let bytes = unsafe { input.as_ptr().add(len).cast::<u64>().read_unaligned() };
                let bytes = bytes ^ 0x0a0a0a0a0a0a0a0a;
                let zero_or_high = bytes.wrapping_sub(0x0101010101010101);
                let not_high = !bytes & 0x8080808080808080;
                let mask = zero_or_high & not_high;

                if mask != 0 {
                    len += (mask.trailing_zeros() / 8) as usize;
                    break;
                }
                len += 8;
            }

            while len < input.len() && input[len] != b'\n' {
                len += 1;
            }

            let line = &input[..len];
            input = &input[len+1..];
            line
        };

        let mut vs = [0, 0];
        let mut state = 0;

        for i in line.iter().copied() {
            let (s, a) = unsafe {
                *fwd.get_unchecked(state as usize * 256 + i as usize)
            };
            state = s;

            //println!("{state} {:?} -> {s} {a}", i as char);

            if a != 0 {
                //println!("a: {a}");
                vs[0] = a as i32;
                break;
            }
        }

        for i in line.iter().rev().copied() {
            let (s, a) = unsafe {
                *bwd.get_unchecked(state as usize * 256 + i as usize)
            };
            state = s;

            //println!("{state} {:?} -> {s} {a}", i as char);

            if a != 0 {
                //println!("a: {a}");
                vs[1] = a as i32;
                break;
            }
        }

        result += vs[0]*10 + vs[1];
    }

    return result;
}

fn part_2_fsm2_vect_threaded(input: &str, fwd: &[(u8, u8)], bwd: &[(u8, u8)]) -> i32 {
    let n = if input.len() < 128 { 1 } else { 12 };

    (0..n)
    .into_par_iter()
    .map(move |i| {
        let chunk_size = input.len() / n;

        let mut begin = chunk_size * i;
        let mut end = chunk_size * (i+1);

        while begin > 0 && input.as_bytes()[begin - 1] != b'\n' {
            begin -= 1;
        }

        if i < n-1 {
            while end > begin && input.as_bytes()[end - 1] != b'\n' {
                end -= 1;
            }
        }
        else {
            end = input.len();
        }

        //println!("{begin}..{end}");

        part_2_fsm2_vect(&input[begin..end], fwd, bwd)
    }).sum()
}


fn part_2_fsm3(input: &str, fwd: &[(u8, u8)], bwd: &[(u8, u8)]) -> i32 {
    let mut input = input.as_bytes();

    let iter = core::iter::from_fn(|| {
        if input.len() == 0 {
            return None;
        }

        let mut len = 0;

        while len + 8 < input.len() {
            let bytes = unsafe { input.as_ptr().add(len).cast::<u64>().read_unaligned() };
            let bytes = bytes ^ 0x0a0a0a0a0a0a0a0a;
            let zero_or_high = bytes.wrapping_sub(0x0101010101010101);
            let not_high = !bytes & 0x8080808080808080;
            let mask = zero_or_high & not_high;

            if mask != 0 {
                len += (mask.trailing_zeros() / 8) as usize;
                break;
            }
            len += 8;
        }

        while len < input.len() && input[len] != b'\n' {
            len += 1;
        }

        let line = &input[..len];
        input = &input[len+1..];
        return Some(line);
    });


    iter.par_bridge().map(|line| {
        let mut vs = [0, 0];
        let mut state = 0;

        for i in line.iter().copied() {
            let (s, a) = unsafe {
                *fwd.get_unchecked(state as usize * 256 + i as usize)
            };
            state = s;

            if a != 0 {
                vs[0] = a as i32;
                break;
            }
        }

        for i in line.iter().rev().copied() {
            let (s, a) = unsafe {
                *bwd.get_unchecked(state as usize * 256 + i as usize)
            };
            state = s;

            if a != 0 {
                vs[1] = a as i32;
                break;
            }
        }

        return vs[0]*10 + vs[1];
    }).sum()
}


fn run(name: &str, f: impl FnOnce(&str) -> i32, input: &str) {
    let t0 = std::time::Instant::now();
    let result = f(input);
    let dt = t0.elapsed();
    println!("{name}: {result} in {dt:?}, {:.2} MiB/s",
        input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}


fn bench(f: impl Fn(&str) -> i32, n: u32, input: &str) {
    let t0 = std::time::Instant::now();
    let mut result = 0;
    for _ in 0..n {
        result = f(input);
    }
    let dt = t0.elapsed();
    println!("result: {result} in {:?}, {:.2} MiB/s",
        dt/n,
        n as f64 * input.len() as f64 / dt.as_secs_f64() / 1024.0 / 1024.0);
}

pub fn main() {
    println!("-- day 01 --");

    run("part_1", part_1, include_str!("d01-test.txt"));
    run("part_1", part_1, include_str!("d01-prod.txt"));

    run("part_2", part_2, include_str!("d01-test-2.txt"));
    run("part_2", part_2, include_str!("d01-prod.txt"));

    let fsm = part_2_make_fsm();
    run("part_2_fsm", |i| part_2_fsm(i, &fsm), include_str!("d01-test-2.txt"));
    run("part_2_fsm", |i| part_2_fsm(i, &fsm), include_str!("d01-prod.txt"));

    run("part_2_fsm_threaded", |i| part_2_fsm_threaded(i, &fsm), include_str!("d01-test-2.txt"));
    run("part_2_fsm_threaded", |i| part_2_fsm_threaded(i, &fsm), include_str!("d01-prod.txt"));

    let (fsm2a, fsm2b) = part_2_make_fsm2();
    run("part_2_fsm2", |i| part_2_fsm2(i, &fsm2a, &fsm2b), include_str!("d01-test-2.txt"));
    run("part_2_fsm2", |i| part_2_fsm2(i, &fsm2a, &fsm2b), include_str!("d01-prod.txt"));

    run("part_2_fsm2_threaded", |i| part_2_fsm2_threaded(i, &fsm2a, &fsm2b), include_str!("d01-test-2.txt"));
    run("part_2_fsm2_threaded", |i| part_2_fsm2_threaded(i, &fsm2a, &fsm2b), include_str!("d01-prod.txt"));

    run("part_2_fsm2_vect", |i| part_2_fsm2_vect(i, &fsm2a, &fsm2b), include_str!("d01-test-2.txt"));
    run("part_2_fsm2_vect", |i| part_2_fsm2_vect(i, &fsm2a, &fsm2b), include_str!("d01-prod.txt"));

    run("part_2_fsm2_vect_threaded", |i| part_2_fsm2_vect_threaded(i, &fsm2a, &fsm2b), include_str!("d01-test-2.txt"));
    run("part_2_fsm2_vect_threaded", |i| part_2_fsm2_vect_threaded(i, &fsm2a, &fsm2b), include_str!("d01-prod.txt"));

    run("part_2_fsm3", |i| part_2_fsm3(i, &fsm2a, &fsm2b), include_str!("d01-test-2.txt"));
    run("part_2_fsm3", |i| part_2_fsm3(i, &fsm2a, &fsm2b), include_str!("d01-prod.txt"));

    println!();


    /*
    let mut big = String::new();
    for _ in 0..1000 {
        big.push_str(include_str!("d01-prod.txt"));
    }
    run(|i| part_2_fsm(i, &fsm), &big);
    run(|i| part_2_fsm_threaded(i, &fsm), &big);
    run(|i| part_2_fsm2(i, &fsm2a, &fsm2b), &big);
    run(|i| part_2_fsm2_threaded(i, &fsm2a, &fsm2b), &big);
    run(|i| part_2_fsm2_vect(i, &fsm2a, &fsm2b), &big);
    run(|i| part_2_fsm2_vect_threaded(i, &fsm2a, &fsm2b), &big);
    run(|i| part_2_fsm3(i, &fsm2a, &fsm2b), &big);
    println!();

    println!("bench part 2 10_000 iters");
    bench(|i| part_2(i), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm(i, &fsm), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm_threaded(i, &fsm), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm2(i, &fsm2a, &fsm2b), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm2_threaded(i, &fsm2a, &fsm2b), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm2_vect(i, &fsm2a, &fsm2b), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm2_vect_threaded(i, &fsm2a, &fsm2b), 10_000, include_str!("d01-prod.txt"));
    bench(|i| part_2_fsm3(i, &fsm2a, &fsm2b), 10_000, include_str!("d01-prod.txt"));
    println!();

    println!("bench part 2 1000x cat'd, 100 iters");
    bench(|i| part_2(i), 100, &big);
    bench(|i| part_2_fsm(i, &fsm), 100, &big);
    bench(|i| part_2_fsm_threaded(i, &fsm), 100, &big);
    bench(|i| part_2_fsm2(i, &fsm2a, &fsm2b), 100, &big);
    bench(|i| part_2_fsm2_threaded(i, &fsm2a, &fsm2b), 100, &big);
    bench(|i| part_2_fsm2_vect(i, &fsm2a, &fsm2b), 100, &big);
    bench(|i| part_2_fsm2_vect_threaded(i, &fsm2a, &fsm2b), 100, &big);
    bench(|i| part_2_fsm3(i, &fsm2a, &fsm2b), 100, &big);
    println!();
    */
}


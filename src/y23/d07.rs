
const NUM_CARDS: usize = 13;

const TYPE_FIVE:  usize = 6;
const TYPE_FOUR:  usize = 5;
const TYPE_HOUSE: usize = 4;
const TYPE_THREE: usize = 3;
const TYPE_2PAIR: usize = 2;
const TYPE_1PAIR: usize = 1;
const TYPE_HIGH:  usize = 0;
const NUM_HAND_TYPES: usize = 7;

fn part_1(input: &str) -> i32 {
    let mut hands: [Vec<([u8; 5], u16)>; NUM_HAND_TYPES] = core::array::from_fn(|_| vec![]);

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        assert!(hand.len() == 5);

        let bid = bid.parse().unwrap();

        let mut counts = [0; NUM_CARDS];
        let hand = core::array::from_fn(|i| {
            let value = match hand.as_bytes()[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' =>  9,
                b'T' =>  8,
                b'9' =>  7,
                b'8' =>  6,
                b'7' =>  5,
                b'6' =>  4,
                b'5' =>  3,
                b'4' =>  2,
                b'3' =>  1,
                b'2' =>  0,
                _ => unreachable!()
            };

            counts[value as usize] += 1;
            return value;
        });

        let mut ty = TYPE_HIGH;
        let mut has_pair = false;
        let mut has_triplet = false;
        for count in counts.iter().copied() {
            match count {
                2 => {
                    if has_pair {
                        ty = TYPE_2PAIR;
                        break;
                    }
                    if has_triplet {
                        ty = TYPE_HOUSE;
                        break;
                    }
                    has_pair = true;
                }

                3 => {
                    if has_pair {
                        ty = TYPE_HOUSE;
                        break;
                    }
                    has_triplet = true;
                }

                4 => {
                    ty = TYPE_FOUR;
                    break;
                }

                5 => {
                    ty = TYPE_FIVE;
                    break;
                }

                _ => (),
            }
        }
        if ty == TYPE_HIGH {
            if has_pair {
                ty = TYPE_1PAIR;
            }
            else if has_triplet {
                ty = TYPE_THREE;
            }
        }

        hands[ty].push((hand, bid));
    }

    let mut result = 0;
    let mut rank = 1;
    for hands in &mut hands {
        hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        for (_, bid) in hands {
            result += rank * *bid as i32;
            rank += 1;
        }
    }

    return result;
}


fn part_2(input: &str) -> i32 {
    let mut hands: [Vec<([u8; 5], u16)>; NUM_HAND_TYPES] = core::array::from_fn(|_| vec![]);

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        assert!(hand.len() == 5);

        let bid = bid.parse().unwrap();

        //let hand_str = hand;

        let mut counts = [0; NUM_CARDS];
        let hand = core::array::from_fn(|i| {
            let value = match hand.as_bytes()[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'T' =>  9,
                b'9' =>  8,
                b'8' =>  7,
                b'7' =>  6,
                b'6' =>  5,
                b'5' =>  4,
                b'4' =>  3,
                b'3' =>  2,
                b'2' =>  1,
                b'J' =>  0,
                _ => unreachable!()
            };

            counts[value as usize] += 1;
            return value;
        });

        let mut js_left = counts[0];

        let mut counts: [_; NUM_CARDS-1] = core::array::from_fn(|i| counts[i+1]);
        counts.sort();

        let mut ty = TYPE_HIGH;
        let mut has_pair = false;
        let mut has_triplet = false;
        for count in counts.iter().copied().rev() {
            match count + js_left {
                0 => break,

                2 => {
                    if has_pair {
                        ty = TYPE_2PAIR;
                        break;
                    }
                    if has_triplet {
                        ty = TYPE_HOUSE;
                        break;
                    }
                    has_pair = true;
                    js_left = 0;
                }

                3 => {
                    if has_pair {
                        ty = TYPE_HOUSE;
                        break;
                    }
                    has_triplet = true;
                    js_left = 0;
                }

                4 => {
                    ty = TYPE_FOUR;
                    break;
                }

                5 => {
                    ty = TYPE_FIVE;
                    break;
                }

                _ => (),
            }
        }
        if ty == TYPE_HIGH {
            if has_pair {
                ty = TYPE_1PAIR;
            }
            else if has_triplet {
                ty = TYPE_THREE;
            }
        }

        //println!("{hand_str}: {counts:?} -> {ty}");

        hands[ty].push((hand, bid));
    }

    let mut result = 0;
    let mut rank = 1;
    for hands in &mut hands {
        hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        for (_, bid) in hands {
            result += rank * *bid as i32;
            rank += 1;
        }
    }

    return result;
}

fn part_2_fast(input: &str) -> i32 {
    let mut hands: [Vec<([u8; 5], u16)>; NUM_HAND_TYPES] = core::array::from_fn(|_| vec![]);

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        assert!(hand.len() == 5);

        let bid = bid.parse().unwrap();

        //let hand_str = hand;

        let mut card_counts = [0; NUM_CARDS];
        let mut multi_counts = [0; 5];
        let mut js = 0;
        let hand = core::array::from_fn(|i| {
            let value = match hand.as_bytes()[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'T' =>  9,
                b'9' =>  8,
                b'8' =>  7,
                b'7' =>  6,
                b'6' =>  5,
                b'5' =>  4,
                b'4' =>  3,
                b'3' =>  2,
                b'2' =>  1,

                b'J' => {
                    js += 1;
                    return 0;
                }

                _ => unreachable!()
            };

            let old_count = card_counts[value as usize];
            card_counts[value as usize] = old_count + 1;

            if old_count != 0 {
                multi_counts[old_count - 1] -= 1;
                multi_counts[old_count    ] += 1;
            }
            else {
                multi_counts[0] += 1;
            }

            return value;
        });

        //println!("{hand_str}: {card_counts:?}, {multi_counts:?}");

        for i in (0..multi_counts.len()).rev() {
            if multi_counts[i] != 0 {
                multi_counts[i] -= 1;
                multi_counts[i + js] += 1;
                js = 0;
                break;
            }
        }
        if js != 0 {
            debug_assert!(js == 5);
            multi_counts[5-1] = 1;
        }

        let ty =
            if multi_counts[5-1] != 0 { TYPE_FIVE }
            else if multi_counts[4-1] != 0 { TYPE_FOUR }
            else if multi_counts[3-1] != 0 {
                if multi_counts[2-1] != 0 { TYPE_HOUSE }
                else                      { TYPE_THREE }
            }
            else if multi_counts[2-1] > 1 { TYPE_2PAIR }
            else if multi_counts[2-1] == 1 { TYPE_1PAIR }
            else { TYPE_HIGH };

        //println!("{hand_str}: {ty}");

        hands[ty].push((hand, bid));
    }

    let mut result = 0;
    let mut rank = 1;
    for hands in &mut hands {
        hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        for (_, bid) in hands {
            result += rank * *bid as i32;
            rank += 1;
        }
    }

    return result;
}

fn part_2_fast_isse(input: &str) -> i32 {
    let mut hands: [Vec<([u8; 5], u16)>; NUM_HAND_TYPES] = core::array::from_fn(|_| vec![]);

    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        assert!(hand.len() == 5);

        let bid = bid.parse().unwrap();

        //let hand_str = hand;

        let mut card_counts = [0; NUM_CARDS];
        let hand = core::array::from_fn(|i| {
            let value = match hand.as_bytes()[i] {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'T' =>  9,
                b'9' =>  8,
                b'8' =>  7,
                b'7' =>  6,
                b'6' =>  5,
                b'5' =>  4,
                b'4' =>  3,
                b'3' =>  2,
                b'2' =>  1,
                b'J' =>  0,
                _ => unreachable!()
            };
            card_counts[value as usize] += 1;
            return value;
        });

        let mut highest_count = 0;
        let mut second_count = 0;
        for count in card_counts.iter().copied().skip(1) {
            if count > highest_count {
                second_count = highest_count;

                highest_count = count;
            } else if count > second_count {
                second_count = count;
            }
        }
        highest_count += card_counts[0];

        let ty = match (second_count, highest_count) {
            (1, 1) => 0,
            (1, 2) => 1,
            (2, 2) => 2,
            (1, 3) => 3,
            (2, 3) => 4,
            (1, 4) => 5,
            (0, 5) => 6,
            _ => panic!(),
        };

        //println!("{hand_str}: {ty}");

        hands[ty].push((hand, bid));
    }

    let mut result = 0;
    let mut rank = 1;
    for hands in &mut hands {
        hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        for (_, bid) in hands {
            result += rank * *bid as i32;
            rank += 1;
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
    println!("-- day 07 --");

    run("part_1", part_1, include_str!("d07-test.txt"));
    run("part_1", part_1, include_str!("d07-prod.txt"));

    run("part_2", part_2, include_str!("d07-test.txt"));
    run("part_2", part_2, include_str!("d07-prod.txt"));

    run("part_2_fast", part_2_fast, include_str!("d07-test.txt"));
    run("part_2_fast", part_2_fast, include_str!("d07-prod.txt"));

    run("part_2_fast_isse", part_2_fast_isse, include_str!("d07-test.txt"));
    run("part_2_fast_isse", part_2_fast_isse, include_str!("d07-prod.txt"));

    println!();
}


use std::collections::HashMap;
use regex::Regex;


#[derive(Clone, Copy, Debug)]
struct Blueprint {
    id: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsi_robot: (u8, u8),
    geode_robot: (u8, u8),
    max_ore_cost: u8,
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut result = Vec::with_capacity(128);

    let re = Regex::new(r"\d+").unwrap();
    for line in input.lines() {
        let mut numbers = re.find_iter(line);
        let mut next = || -> u8 {
            let number = numbers.next().unwrap();
            number.as_str().parse().unwrap()
        };

        let id = next();
        let ore_robot = next();
        let clay_robot = next();
        let obsi_robot = (next(), next());
        let geode_robot = (next(), next());
        result.push(Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsi_robot,
            geode_robot,
            max_ore_cost: ore_robot.max(clay_robot).max(obsi_robot.0).max(geode_robot.0),
        });
        assert!(numbers.next().is_none());
    }

    result
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pack {
    ore_robot:   u8,
    clay_robot:  u8,
    obsi_robot:  u8,
    geode_robot: u8,
    ore:   u8,
    clay:  u8,
    obsi:  u8,
    geode: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    minute: u8,
    pack:   Pack,
}

impl State {
    #[inline]
    fn new() -> State {
        State {
            minute: 0,
            pack: Pack {
                ore_robot:   1,
                clay_robot:  0,
                obsi_robot:  0,
                geode_robot: 0,
                ore:   0,
                clay:  0,
                obsi:  0,
                geode: 0,
            },
        }
    }

    #[inline]
    fn can_build_ore_robot(&self, bp: &Blueprint) -> bool {
        self.pack.ore >= bp.ore_robot
    }

    #[inline]
    fn can_build_clay_robot(&self, bp: &Blueprint) -> bool {
        self.pack.ore >= bp.clay_robot
    }

    #[inline]
    fn can_build_obsi_robot(&self, bp: &Blueprint) -> bool {
           self.pack.ore  >= bp.obsi_robot.0
        && self.pack.clay >= bp.obsi_robot.1
    }

    #[inline]
    fn can_build_geode_robot(&self, bp: &Blueprint) -> bool {
           self.pack.ore  >= bp.geode_robot.0
        && self.pack.obsi >= bp.geode_robot.1
    }

    #[inline]
    fn build_ore_robot(self, bp: &Blueprint) -> Self {
        let mut result = self;
        result.pack.ore -= bp.ore_robot;
        result.pack.ore_robot += 1;
        return result;
    }

    #[inline]
    fn build_clay_robot(self, bp: &Blueprint) -> Self {
        let mut result = self;
        result.pack.ore -= bp.clay_robot;
        result.pack.clay_robot += 1;
        return result;
    }

    #[inline]
    fn build_obsi_robot(self, bp: &Blueprint) -> Self {
        let mut result = self;
        result.pack.ore  -= bp.obsi_robot.0;
        result.pack.clay -= bp.obsi_robot.1;
        result.pack.obsi_robot += 1;
        return result;
    }

    #[inline]
    fn build_geode_robot(self, bp: &Blueprint) -> Self {
        let mut result = self;
        result.pack.ore  -= bp.geode_robot.0;
        result.pack.obsi -= bp.geode_robot.1;
        result.pack.geode_robot += 1;
        return result;
    }

    #[inline]
    fn step(&self) -> Self {
        let mut this = *self;
        this.minute += 1;
        this.pack.ore   += this.pack.ore_robot;
        this.pack.clay  += this.pack.clay_robot;
        this.pack.obsi  += this.pack.obsi_robot;
        this.pack.geode += this.pack.geode_robot;
        return this;
    }
}


struct Solver {
    bp: Blueprint,
    #[allow(unused)]
    memo: HashMap<u64, (u8, u8)>,
    limit: u8,
    max_result: u8,
}

impl Solver {
    fn new(bp: Blueprint, limit: u8) -> Solver {
        Solver {
            bp,
            memo: HashMap::new(),
            limit,
            max_result: 0,
        }
    }

    fn rec(&mut self, state: State, can_ore: bool, can_clay: bool, can_obsi: bool) -> u8 {
        // let pack_64: u64 = unsafe { core::mem::transmute(state.pack) };

        // if let Some((minute, result)) = self.memo.get(&pack_64).copied() {
        //     if state.minute >= minute {
        //         return result;
        //     }
        // }

        if state.minute == self.limit {
            let result = state.pack.geode;
            //self.memo.insert(pack_64, (state.minute, result));
            self.max_result = self.max_result.max(result);
            return result;
        }

        // can we even beat the max anymore?
        {
            let remaining = (self.limit - state.minute) as u32;
            let max_yield = remaining * state.pack.geode_robot as u32 + (remaining)*(remaining-1)/2;
            if state.pack.geode as u32 + max_yield <= self.max_result as u32 {
                //self.memo.insert(pack_64, (state.minute, 0));
                return 0;
            }
        }

        let mut result = 0;

        if state.can_build_geode_robot(&self.bp) {
            result = result.max(self.rec(state.step().build_geode_robot(&self.bp), true, true, true));
        }
        else {
            let mut new_can_ore = true;
            if state.can_build_ore_robot(&self.bp) {
                new_can_ore = false;

                if can_ore && state.pack.ore_robot < self.bp.max_ore_cost {
                    result = result.max(self.rec(state.step().build_ore_robot(&self.bp), true, true, true));
                }
            }

            let mut new_can_clay = true;
            if state.can_build_clay_robot(&self.bp) {
                new_can_clay = false;

                if can_clay && state.pack.clay_robot < self.bp.obsi_robot.1 {
                    result = result.max(self.rec(state.step().build_clay_robot(&self.bp), true, true, true));
                }
            }

            let mut new_can_obsi = true;
            if state.can_build_obsi_robot(&self.bp) {
                new_can_obsi = false;

                if can_obsi && state.pack.obsi_robot < self.bp.geode_robot.1 {
                    result = result.max(self.rec(state.step().build_obsi_robot(&self.bp), true, true, true));
                }
            }

            // wait & build on next turn.
            result = result.max(self.rec(state.step(), new_can_ore, new_can_clay, new_can_obsi));
        }

        // for some reason, this is equivalent to `self.memo.insert(pack64, (state.minute, result))`
        // but the perf is the same, so who cares.
        // self.memo.entry(pack_64)
        // .and_modify(|(min, res)| {
        //     if result >= *res {
        //         *min = (*min).min(state.minute);
        //         *res = result;
        //     }
        // })
        // .or_insert((state.minute, result));

        return result;
    }
}


fn part_1(input: &str) {
    let blueprints = parse(input);

    let t0 = std::time::Instant::now();
    let mut result = 0;
    for bp in &blueprints {
        let mut solver = Solver::new(*bp, 24);
        let geodes = solver.rec(State::new(), true, true, true);
        //println!("{}: {}", bp.id, geodes);
        result += bp.id as u32 * geodes as u32;
    }
    println!("part 1 result: {} in {:?}", result, t0.elapsed());
}

fn part_2(input: &str) {
    let mut blueprints = parse(input);
    if blueprints.len() > 3 {
        blueprints.truncate(3);
    }

    let t0 = std::time::Instant::now();
    let mut result = 1;
    for bp in &blueprints {
        let mut solver = Solver::new(*bp, 32);
        let geodes = solver.rec(State::new(), true, true, true);
        //println!("{}: {}", bp.id, geodes);
        result *= geodes as u32;
    }
    println!("part 2 result: {} in {:?}", result, t0.elapsed());
}

pub fn main() {
    part_1(include_str!("d19-test.txt"));
    part_2(include_str!("d19-test.txt"));

    part_1(include_str!("d19-prod.txt"));
    part_2(include_str!("d19-prod.txt"));

    part_1(include_str!("d19-prod-2.txt"));
    part_2(include_str!("d19-prod-2.txt"));
}


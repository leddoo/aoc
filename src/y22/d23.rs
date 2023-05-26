
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir { N, S, W, E }


#[derive(Clone, Copy)]
#[repr(align(4))]
struct Cell {
    value: bool,
    moves: u8,
    dir: Option<Dir>,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rect {
    x0: i32,
    y0: i32,
    x1: i32, // exclusive
    y1: i32, // exclusive
}

impl Rect {
    pub const ZERO: Rect = Rect { x0: 0, y0: 0, x1: 0, y1: 0 };

    pub const MAX_MIN: Rect = Rect { x0: i32::MAX, y0: i32::MAX, x1: i32::MIN, y1: i32::MIN };

    #[must_use]
    #[inline(always)]
    fn include(&self, x: i32, y: i32) -> Rect {
        Rect {
            x0: self.x0.min(x),
            y0: self.y0.min(y),
            x1: self.x1.max(x + 1),
            y1: self.y1.max(y + 1),
        }
    }

    #[inline(always)]
    fn w(&self) -> i32 {
        self.x1 - self.x0
    }

    #[inline(always)]
    fn h(&self) -> i32 {
        self.y1 - self.y0
    }
}


struct Elf {
    index: u32,
    x: i32,
    y: i32,
}

struct Board {
    cells: Vec<Cell>,
    w: usize,
    rect: Rect,
    aabb: Rect,

    dirs: [Dir; 4],
    dir_rot: u32,
    elves: Vec<Elf>,
}

impl Board {
    fn new() -> Self {
        Board { 
            cells: vec![], 
            w: 0, 
            rect: Rect::ZERO, 
            aabb: Rect::ZERO, 
            dirs: [Dir::N, Dir::S, Dir::W, Dir::E],
            dir_rot: 0,
            elves: vec![],
        }
    }

    fn resize(&mut self, new_rect: Rect) {
        debug_assert!(new_rect.x0 <= self.rect.x0);
        debug_assert!(new_rect.y0 <= self.rect.y0);
        debug_assert!(new_rect.x1 >= self.rect.x1);
        debug_assert!(new_rect.y1 >= self.rect.y1);

        let new_w = new_rect.w() as usize;
        let new_h = new_rect.h() as usize;

        let old_w = self.rect.w() as usize;
        let dx = (self.rect.x0 - new_rect.x0) as usize;

        let mut new_cells = vec![Cell { value: false, moves: 0, dir: None }; new_w*new_h];
        for y in self.rect.y0..self.rect.y1 {
            let s0 = ((y - self.rect.y0) as usize)*old_w;
            let d0 = ((y - new_rect.y0)  as usize)*new_w + dx;

            for i in 0..old_w {
                new_cells[d0 + i] = self.cells[s0 + i];
            }
        }

        for elf in &mut self.elves {
            elf.index = ((elf.y - new_rect.y0) as usize * new_w + (elf.x - new_rect.x0) as usize) as u32;
        }

        self.cells = new_cells;
        self.w = new_w;
        self.rect = new_rect;
    }

    #[inline(always)]
    fn index(&self, x: i32, y: i32) -> usize {
        ((y - self.rect.y0) as usize)*self.w + (x - self.rect.x0) as usize
    }

    fn set_slow(&mut self, x: i32, y: i32) {
        if self.rect.w() == 0 {
            self.rect = Rect {
                x0: x, y0: y,
                x1: x, y1: y,
            };
            self.resize(Rect {
                x0: x,     y0: y,
                x1: x + 1, y1: y + 1,
            });
            self.aabb = Rect { x0: x, y0: y, x1: x + 1, y1: y + 1 };
        }
        else {
            let new_rect = self.rect.include(x, y);
            if new_rect != self.rect {
                self.resize(new_rect);
            }
            self.aabb = self.aabb.include(x, y);
        }

        let i = self.index(x, y);
        let cell = &mut self.cells[i];
        if !cell.value {
            cell.value = true;
            self.elves.push(Elf { index: i as u32, x, y });
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let dx = (self.aabb.x0 - self.rect.x0) as usize;
        let dy = (self.aabb.y0 - self.rect.y0) as usize;

        for y in 0..self.aabb.h() as usize {
            let i0 = (y + dy)*self.w + dx;
            for x in 0..self.aabb.w() as usize {
                print!("{}", if self.cells[i0 + x].value { "#" } else { "." });
            }
            println!();
        }
        println!();
    }

    fn step(&mut self) -> bool {
        // reserve for step.
        if self.aabb.x0 == self.rect.x0
        || self.aabb.y0 == self.rect.y0
        || self.aabb.x1 == self.rect.x1
        || self.aabb.y1 == self.rect.y1
        {
            let w = self.aabb.w();
            let h = self.aabb.h();
            self.resize(Rect {
                x0: self.aabb.x0 - w,
                y0: self.aabb.y0 - h,
                x1: self.aabb.x1 + w,
                y1: self.aabb.y1 + h,
            });
        }
        debug_assert!(self.aabb.x0 >= self.rect.x0);
        debug_assert!(self.aabb.y0 >= self.rect.y0);
        debug_assert!(self.aabb.x1 <= self.rect.x1);
        debug_assert!(self.aabb.y1 <= self.rect.y1);

        let dirs = self.dirs;
        let dir_rot = self.dir_rot;

        let width = self.w;
        let deltas = [
            -(width as isize),
            width as isize,
            -1,
            1,
        ];
        let deltas_pos = [
            (0, -1),
            (0,  1),
            (-1, 0),
            ( 1, 0),
        ];

        // compute dirs & step counts.
        for elf in &self.elves {
            let i = elf.index as usize;

            let se = self.cells[i + width + 1].value;
            let s  = self.cells[i + width].value;
            let sw = self.cells[i + width - 1].value;
            let ne = self.cells[i - width + 1].value;
            let n  = self.cells[i - width].value;
            let nw = self.cells[i - width - 1].value;
            let e  = self.cells[i + 1].value;
            let w  = self.cells[i - 1].value;

            if !se && !s && !sw && !ne && !n && !nw && !e && !w {
                continue;
            }

            let can_n = !ne && !n && !nw;
            let can_s = !se && !s && !sw;
            let can_w = !nw && !w && !sw;
            let can_e = !ne && !e && !se;

            /*
            let cans = [can_n, can_s, can_w, can_e];

            let mut the_dir = None;
            for dir in dirs {
                if cans[dir as usize] {
                    self.cells[i].dir = Some(dir);
                    self.cells[(i as isize + deltas[dir as usize]) as usize].moves += 1;
                    break;
                }
            }
            */

            let cans = ((can_n as u32) << 0) | ((can_s as u32) << 8) | ((can_w as u32) << 16) | ((can_e as u32) << 24);
            let cans = cans.rotate_right(dir_rot * 8);
            if cans != 0 {
                let dir_index = cans.trailing_zeros() / 8;
                let dir = dirs[dir_index as usize];
                self.cells[i].dir = Some(dir);
                self.cells[(i as isize + deltas[dir as usize]) as usize].moves += 1;
            }

            // removing these bounds checks takes it from 30.5 ms to 28.5 ms.
            /*
            unsafe {
                let se = self.cells.get_unchecked(i + width + 1).value;
                let s  = self.cells.get_unchecked(i + width).value;
                let sw = self.cells.get_unchecked(i + width - 1).value;
                let ne = self.cells.get_unchecked(i - width + 1).value;
                let n  = self.cells.get_unchecked(i - width).value;
                let nw = self.cells.get_unchecked(i - width - 1).value;
                let e  = self.cells.get_unchecked(i + 1).value;
                let w  = self.cells.get_unchecked(i - 1).value;

                if !se && !s && !sw && !ne && !n && !nw && !e && !w {
                    continue;
                }

                let can_n = !ne && !n && !nw;
                let can_s = !se && !s && !sw;
                let can_w = !nw && !w && !sw;
                let can_e = !ne && !e && !se;

                let cans = ((can_n as u32) << 0) | ((can_s as u32) << 8) | ((can_w as u32) << 16) | ((can_e as u32) << 24);
                let cans = cans.rotate_right(dir_rot * 8);
                if cans != 0 {
                    let dir_index = cans.trailing_zeros() / 8;
                    let dir = dirs[dir_index as usize];
                    self.cells[i].dir = Some(dir);
                    self.cells.get_unchecked_mut((i as isize + deltas[dir as usize]) as usize).moves += 1;
                }
            }
            */
        }

        // move elves.
        self.aabb = Rect::MAX_MIN;
        let mut num_moves = 0;
        for i in 0..self.elves.len() {
            let elf = &mut self.elves[i];
            let old_i = elf.index as usize;

            if let Some(dir) = self.cells[old_i].dir.take() {
                let new_i = (old_i as isize + deltas[dir as usize]) as usize;

                let target = &mut self.cells[new_i];
                if target.moves == 1 {
                    num_moves += 1;

                    *target = Cell { value: true, moves: 0, dir: None };
                    self.cells[old_i].value = false;

                    elf.index = new_i as u32;

                    let (dx, dy) = deltas_pos[dir as usize];
                    elf.x += dx;
                    elf.y += dy;
                }
                else {
                    target.moves = 0;
                }
            }
            self.aabb = self.aabb.include(elf.x, elf.y);
        }

        self.dirs.rotate_left(1);
        self.dir_rot += 1;
        self.dir_rot %= 4;

        num_moves != 0
    }

    fn empty_tiles(&self) -> u32 {
        let w = self.aabb.w() as u32;
        let h = self.aabb.h() as u32;
        w*h - self.elves.len() as u32
    }
}

fn parse(input: &str) -> Board {
    let mut board = Board::new();
    for (y, line) in input.split("\n").enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            if ch == '#' as u8 {
                board.set_slow(x as i32, y as i32);
            }
        }
    }
    board
}

fn part_1(input: &str) {
    let t0 = std::time::Instant::now();

    let mut board = parse(input);
    for _ in 0..10 {
        board.step();
    }
    println!("part 1: {} in {:?}", board.empty_tiles(), t0.elapsed());
}

fn part_2(input: &str) {
    //for _ in 0..100 {
    let t0 = std::time::Instant::now();

    let mut board = parse(input);
    let mut i = 1;
    while board.step() {
        i += 1;
    }
    println!("part 2: {} in {:?}", i, t0.elapsed());
    //}
}

pub fn main() {
    part_1(include_str!("d23-test.txt"));
    part_2(include_str!("d23-test.txt"));
    part_1(include_str!("d23-prod.txt"));
    part_2(include_str!("d23-prod.txt"));
    part_1(include_str!("d23-prod-2.txt"));
    part_2(include_str!("d23-prod-2.txt"));
}


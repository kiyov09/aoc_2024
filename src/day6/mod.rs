use std::{collections::HashSet, fs::read_to_string};

const PATH: &str = "./inputs/day6/input.txt";

const GUARD_INIT: char = '^';
const OBSTACLE: char = '#';

type Pos = (usize, usize);

/// Where the guard is looking at
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

/// The state of the guard
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Guard {
    pos: Pos,
    facing: Facing,
}

impl Guard {
    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Facing::Up => Facing::Right,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Right => Facing::Down,
        }
    }

    fn next_pos(&self) -> Pos {
        match self.facing {
            Facing::Up => (self.pos.0 - 1, self.pos.1),
            Facing::Down => (self.pos.0 + 1, self.pos.1),
            Facing::Left => (self.pos.0, self.pos.1 - 1),
            Facing::Right => (self.pos.0, self.pos.1 + 1),
        }
    }

    fn is_done(&self, map: &[Vec<char>]) -> bool {
        let row_len = map[0].len();
        let col_len = map.len();

        match &self.facing {
            Facing::Up if self.pos.0 == 0 => true,
            Facing::Down if self.pos.0 == col_len - 1 => true,
            Facing::Left if self.pos.1 == 0 => true,
            Facing::Right if self.pos.1 == row_len - 1 => true,
            _ => false,
        }
    }

    /// Make the guard move till she reaches the end of the map
    fn do_round(&mut self, map: Vec<Vec<char>>) -> usize {
        let mut set = HashSet::new();

        set.insert(self.pos);

        loop {
            let next_pos = self.next_pos();

            if map[next_pos.0][next_pos.1] == OBSTACLE {
                self.turn_right();
                continue;
            }

            self.pos = next_pos;
            set.insert(self.pos);

            if self.is_done(&map) {
                return set.len();
            }
        }
    }

    /// Make the guard move till she reaches the end of the map (no loop) or she gets to a position
    /// she has already been (loop)
    fn is_loop(&mut self, map: &[Vec<char>]) -> bool {
        let mut visited = HashSet::new();
        visited.reserve(map.len());

        loop {
            let next_pos = self.next_pos();

            if visited.contains(self) {
                return true;
            }

            visited.insert(self.clone());

            if map[next_pos.0][next_pos.1] == OBSTACLE {
                self.turn_right();
                continue;
            }

            self.pos = next_pos;

            if self.is_done(map) {
                return false;
            }
        }
    }
}

pub fn task_1() {
    let input = read_to_string(PATH).expect("can't read the input");
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut pos = (0, 0);

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == GUARD_INIT {
                pos = (row, col);
                break;
            }
        }
    }

    let mut guard = Guard {
        pos,
        facing: Facing::Up,
    };

    let positions = guard.do_round(map);

    // assert_eq!(5516, positions);

    println!("{positions}");
}

pub fn task_2() {
    let input = read_to_string(PATH).expect("can't read the input");
    let mut map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut pos = (0, 0);

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == GUARD_INIT {
                pos = (row, col);
                break;
            }
        }
    }

    let mut loops = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if (row, col) == pos {
                continue;
            }

            let temp = map[row][col];
            map[row][col] = OBSTACLE;

            let mut guard = Guard {
                pos,
                facing: Facing::Up,
            };

            if guard.is_loop(&map) {
                loops += 1;
            }

            map[row][col] = temp
        }
    }

    // this line was added after I got the right answer
    assert_eq!(2008, loops);

    println!("{loops}");
}

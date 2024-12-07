use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    position: (u32, u32),
    direction: Direction,
}

impl State {
    const fn new(position: (u32, u32), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    pos: (u32, u32),
    states: Vec<State>,
    direction: Direction,
}

impl Guard {
    fn from_map(map: &str) -> Self {
        let starting_pos = map
            .split('\n')
            .enumerate()
            .find_map(|(n, l)| {
                if l.contains('^') {
                    Some((
                        l.chars().position(|c| c != '.' && c != '#').unwrap() as u32,
                        n as u32,
                    ))
                } else {
                    None
                }
            })
            .unwrap();
        Self {
            pos: starting_pos,
            states: vec![State::new(starting_pos, Direction::Up)],
            direction: Direction::Up,
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Floor,
    Obstacle,
    Guard,
}

impl Tile {
    const fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            '#' => Self::Obstacle,
            _ => Self::Guard,
        }
    }
}

#[derive(Clone)]
struct Map {
    lines: Vec<Vec<Tile>>,
    guard: Guard,
}

impl Map {
    fn from_str(map: &str) -> Self {
        Self {
            lines: map
                .lines()
                .map(|l| l.chars().map(Tile::from_char).collect::<Vec<Tile>>())
                .collect::<Vec<Vec<Tile>>>(),
            guard: Guard::from_map(map),
        }
    }

    fn with_obstacle(&self, position: (usize, usize)) -> Self {
        Self {
            lines: self
                .lines
                .iter()
                .enumerate()
                .map(|(y, l)| {
                    l.iter()
                        .enumerate()
                        .map(|(x, t)| {
                            if x == position.0 && y == position.1 {
                                Tile::Obstacle
                            } else {
                                *t
                            }
                        })
                        .collect()
                })
                .collect(),
            guard: self.guard.clone(),
        }
    }

    fn guard_walk(&mut self) -> Option<bool> {
        let (next_x, next_y) = match self.guard.direction {
            Direction::Left => (self.guard.pos.0.checked_sub(1)?, self.guard.pos.1),
            Direction::Right => (self.guard.pos.0 + 1, self.guard.pos.1),
            Direction::Up => (self.guard.pos.0, self.guard.pos.1.checked_sub(1)?),
            Direction::Down => (self.guard.pos.0, self.guard.pos.1 + 1),
        };

        let next_tile = self.lines.get(next_y as usize)?.get(next_x as usize)?;

        if matches!(next_tile, Tile::Obstacle) {
            self.guard.rotate();

            return Some(false);
        }

        self.guard.pos = (next_x, next_y);

        self.guard
            .states
            .push(State::new(self.guard.pos, self.guard.direction));
        Some(true)
    }
}

pub fn day6() {
    let buf = fs::read("src/day6_input.txt").expect("Couldn't read input");
    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let mut map = Map::from_str(&input);
    while map.guard_walk().is_some() {}

    let obstacle_map = Map::from_str(&input);

    let positions = map
        .guard
        .states
        .iter()
        .filter_map(|s| {
            let mut new_map =
                obstacle_map.with_obstacle((s.position.0 as usize, s.position.1 as usize));

            while new_map.guard_walk().is_some() {
                if new_map.guard.states.iter().unique().count() != new_map.guard.states.len() {
                    return Some(s.position);
                }
            }
            None
        })
        .unique()
        .count();

    println!(
        "DAY 6:\n\t{}\n\t{positions}",
        map.guard.states.iter().map(|s| s.position).unique().count()
    );
}

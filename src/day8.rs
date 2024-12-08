use itertools::Itertools;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Antenna {
    pos: (i32, i32),
    char: char,
}

impl Antenna {
    const fn new(pos: (i32, i32), char: char) -> Self {
        Self { pos, char }
    }
}

pub fn day8() {
    let buf = fs::read("src/day8_input.txt").expect("Couldn't read input");
    let mut map = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    map.pop();

    let antennas = map
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(Antenna::new((x as i32, y as i32), c))
                    }
                })
                .collect::<Vec<Antenna>>()
        })
        .collect::<Vec<Antenna>>();

    let grouped = antennas
        .iter()
        .map(|a| a.char)
        .unique()
        .map(|c| antennas.iter().filter(|a| a.char == c).collect_vec())
        .collect_vec();

    let antinodes = grouped
        .iter()
        .flat_map(|g| {
            g.iter()
                .combinations(2)
                .flat_map(|k| {
                    let (x1, y1) = k.first().unwrap().pos;
                    let (x2, y2) = k.last().unwrap().pos;
                    let (dx, dy) = ((x2 - x1), (y2 - y1));

                    [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)]
                })
                .collect_vec()
        })
        .filter(|(x, y)| {
            *x < map.lines().next().unwrap().len() as i32
                && *y < map.lines().count() as i32
                && *x >= 0
                && *y >= 0
        })
        .unique()
        .collect_vec();

    let extended_antinodes = grouped
        .iter()
        .flat_map(|g| {
            g.iter()
                .combinations(2)
                .flat_map(|k| {
                    let (x1, y1) = k.first().unwrap().pos;
                    let (x2, y2) = k.last().unwrap().pos;

                    let mut ans: Vec<(i32, i32)> = vec![];
                    let (dx, dy) = ((x2 - x1), (y2 - y1));

                    let map_width = map.lines().next().unwrap().len() as i32;

                    for i in 0..map_width {
                        let an = [
                            (x1 - (i * dx), y1 - (i * dy)),
                            (x2 + (i * dx), y2 + (i * dy)),
                        ];

                        let an0_valid = an[0].0 < map_width
                            && an[0].1 < map.lines().count() as i32
                            && an[0].0 >= 0
                            && an[0].1 >= 0;

                        let an1_valid = an[1].0 < map_width
                            && an[1].1 < map.lines().count() as i32
                            && an[1].0 >= 0
                            && an[1].1 >= 0;

                        if an0_valid {
                            ans.push(an[0]);
                        }
                        if an1_valid {
                            ans.push(an[1]);
                        }
                        if !an1_valid && !an0_valid {
                            break;
                        }
                    }

                    ans
                })
                .collect_vec()
        })
        .unique()
        .collect_vec();

    println!(
        "DAY 8:\n\tAntinodes: {}\n\tAntinodes, considering resonant harmonics: {}",
        antinodes.len(),
        extended_antinodes.len(),
    );
}

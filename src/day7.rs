use itertools::{repeat_n, Itertools};
use std::fs;

pub fn day7() {
    let buf = fs::read("src/day7_input.txt").expect("Couldn't read input");
    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let sum = input
        .split('\n')
        .filter_map(|l| {
            let value = l
                .split(':')
                .next()
                .unwrap()
                .parse::<u64>()
                .expect("Couldn't parse equation");
            let ints = l
                .split(' ')
                .skip(1)
                .map(|n| n.parse::<u64>().expect("Couldn't parse equation"))
                .collect::<Vec<u64>>();

            let mut solveable = repeat_n(['+', '*'].iter(), ints.len() - 1)
                .multi_cartesian_product()
                .map(|c| {
                    let mut sum = vec![*ints.first().unwrap()];

                    c.iter().enumerate().for_each(|(n, o)| match o {
                        '+' => sum.push(*sum.last().unwrap() + *ints.get(n + 1).unwrap()),
                        '*' => sum.push(*sum.last().unwrap() * *ints.get(n + 1).unwrap()),
                        _ => (),
                    });

                    sum
                });

            if solveable.any(|s| *s.last().unwrap() == value) {
                Some(value)
            } else {
                None
            }
        })
        .sum::<u64>();

    let total_sum = input
        .split('\n')
        .filter_map(|l| {
            let value = l
                .split(':')
                .next()
                .unwrap()
                .parse::<u64>()
                .expect("Couldn't parse equation");
            let ints = l
                .split(' ')
                .skip(1)
                .map(|n| n.parse::<u64>().expect("Couldn't parse equation"))
                .collect::<Vec<u64>>();

            let mut solveable = repeat_n(['+', '*', '|'].iter(), ints.len() - 1)
                .multi_cartesian_product()
                .map(|c| {
                    let mut sum = vec![*ints.first().unwrap()];

                    c.iter().enumerate().for_each(|(n, o)| match o {
                        '+' => sum.push(*sum.last().unwrap() + *ints.get(n + 1).unwrap()),
                        '*' => sum.push(*sum.last().unwrap() * *ints.get(n + 1).unwrap()),
                        '|' => sum.push(
                            (sum.last().unwrap().to_string()
                                + &ints.get(n + 1).unwrap().to_string())
                                .parse::<u64>()
                                .expect("Couldn't concatenate"),
                        ),
                        _ => (),
                    });

                    sum
                });

            if solveable.any(|s| *s.last().unwrap() == value) {
                Some(value)
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("DAY 7:\n\tSum with *, +: {sum:?}\n\tSum with *, +, |: {total_sum:?}");
}

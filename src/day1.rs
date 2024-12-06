use std::fs;

pub fn day1() {
    let buf = fs::read("src/day1_input.txt").expect("Couldn't read input");
    let mut string = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    string.pop();

    let codes = string
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .copied()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|v| {
            vec![
                str::parse::<i32>(v.first().unwrap()).unwrap(),
                str::parse::<i32>(v.last().unwrap()).unwrap(),
            ]
        })
        .collect::<Vec<Vec<_>>>();

    let mut left = codes
        .iter()
        .map(|v| *v.first().expect("Empty Vec"))
        .collect::<Vec<_>>();
    left.sort_unstable();

    let mut right = codes
        .iter()
        .map(|v| *v.last().expect("Empty Vec"))
        .collect::<Vec<_>>();
    right.sort_unstable();

    let distance = (0..codes.len())
        .map(|i| {
            (left.get(i).expect("Couldn't index left")
                - right.get(i).expect("Couldn't index right"))
            .abs()
        })
        .sum::<i32>();

    println!("DAY 1:\n\tDistance: {distance}");

    let similarity = left
        .iter()
        .map(|l| l * (right.split(|r| *r == *l).count() as i32 - 1))
        .sum::<i32>();

    println!("\tSimilarity: {similarity}");
}

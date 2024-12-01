use std::fs;

pub fn day1() {
    let buf = fs::read("src/day1_input.txt").expect("Couldn't read input");
    let mut string = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    string.pop();

    let codes = string
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .cloned()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>()
        .into_iter()
        .map(|v| {
            vec![
                str::parse::<i32>(*v.first().unwrap()).unwrap(),
                str::parse::<i32>(*v.last().unwrap()).unwrap(),
            ]
        })
        .collect::<Vec<Vec<_>>>();

    let mut left = codes
        .iter()
        .map(|v| *v.first().expect("Empty Vec"))
        .collect::<Vec<_>>();
    left.sort();

    let mut right = codes
        .iter()
        .map(|v| *v.last().expect("Empty Vec"))
        .collect::<Vec<_>>();
    right.sort();

    let sorted_codes = (0..=codes.len() - 1)
        .map(|u| {
            vec![
                left.get(u).expect("Couldn't index left"),
                right.get(u).expect("Couldn't index right"),
            ]
        })
        .collect::<Vec<Vec<_>>>();

    let distance = sorted_codes
        .iter()
        .map(|v| (*v.first().unwrap() - *v.last().unwrap()).abs())
        .sum::<i32>();

    println!("{distance}");

    let similarity = left
        .iter()
        .map(|l| l * (right.split(|r| *r == *l).collect::<Vec<_>>().len() as i32 - 1))
        .sum::<i32>();

    println!("{similarity}");
}

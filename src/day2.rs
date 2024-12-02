use std::fs;

pub fn day2() {
    let buf = fs::read("src/day2_input.txt").expect("Couldn't read input");
    let mut string = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    string.pop();

    let safe = string
        .split('\n')
        .map(|r| {
            r.split(' ')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .iter()
        .map(|r| {
            r.iter()
                .zip(r.iter().skip(1))
                .map(|(l1, l2)| l2 - l1)
                .collect::<Vec<i32>>()
        })
        .filter(is_safe)
        .count();

    println!("DAY 2:\n\tSafe reports: {safe:?}");

    let fixed_safe = string
        .split('\n')
        .map(|r| {
            r.split(' ')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .iter()
        .filter(|r| {
            let d = r
                .iter()
                .zip(r.iter().skip(1))
                .map(|(l1, l2)| l2 - l1)
                .collect::<Vec<i32>>();

            d.iter().filter(|i| **i < 0).count() != d.len()
                && d.iter().filter(|i| **i > 0).count() != d.len()
                || d.iter().any(|d| d.abs() > 3 || d.abs() < 1)
        })
        .filter(|r| {
            (0..r.len())
                .map(|n| {
                    let skip_n = &r
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != n)
                        .map(|(_, v)| *v)
                        .collect::<Vec<i32>>();

                    is_safe(
                        &skip_n
                            .iter()
                            .zip(skip_n.iter().skip(1))
                            .map(|(l1, l2)| l2 - l1)
                            .collect::<Vec<i32>>(),
                    )
                })
                .any(|b| b)
        })
        .count();

    println!("\tSafe reports after fixing: {}", safe + fixed_safe);
}

fn is_safe(l: &Vec<i32>) -> bool {
    (l.iter().filter(|i| **i < 0).count() == l.len()
        || l.iter().filter(|i| **i > 0).count() == l.len())
        && !l.iter().any(|d| d.abs() > 3 || d.abs() < 1)
}

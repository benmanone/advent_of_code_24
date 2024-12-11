use std::collections::HashMap;

pub fn day11() {
    let input = "0 5601550 3914 852 50706 68 6 645371";

    let mut stones: HashMap<String, u64> = HashMap::new();

    for s in input.split(' ') {
        stones
            .entry(s.to_string())
            .and_modify(|f| *f += 1)
            .or_insert(1);
    }

    for _ in 0..75 {
        stones = stones
            .iter()
            .flat_map(|(s, f)| {
                if s == "0" {
                    vec![("1".to_string(), *f)]
                } else if s.len() % 2 == 0 {
                    let split = s.split_at(s.to_string().len() / 2);
                    vec![
                        (split.0.to_string(), *f),
                        if split.1.chars().all(|c| c == '0') {
                            ("0".to_owned(), *f)
                        } else {
                            (split.1.trim_start_matches('0').to_owned(), *f)
                        },
                    ]
                } else {
                    vec![((s.parse::<u64>().unwrap() * 2024).to_string(), *f)]
                }
            })
            .fold(HashMap::new(), |mut acc, (s, f)| {
                *acc.entry(s).or_insert(0) += f;
                acc
            });
    }

    println!("{:?}", stones.into_values().sum::<u64>());
}

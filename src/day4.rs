use std::fs;

pub fn day4() {
    let buf = fs::read("src/day4_input.txt").expect("Couldn't read input");

    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let lines = input.split('\n');

    let count = lines
        .clone()
        .enumerate()
        .zip(lines.clone().skip(1))
        .zip(lines.clone().skip(2))
        .zip(lines.clone().skip(3))
        .map(|((((cl, line1), line2), line3), line4)| {
            line1
                .chars()
                .enumerate()
                .map(|(n, _)| {
                    let down_word = line1.chars().nth(n).unwrap().to_string()
                        + &line2.chars().nth(n).unwrap().to_string()
                        + &line3.chars().nth(n).unwrap().to_string()
                        + &line4.chars().nth(n).unwrap().to_string();

                    down_word.matches("XMAS").count()
                        + down_word.matches("SAMX").count()
                        + if n < line1.len() - 3 {
                            let rdiag_word = line1.chars().nth(n).unwrap().to_string()
                                + &line2.chars().nth(n + 1).unwrap().to_string()
                                + &line3.chars().nth(n + 2).unwrap().to_string()
                                + &line4.chars().nth(n + 3).unwrap().to_string();

                            rdiag_word.matches("XMAS").count() + rdiag_word.matches("SAMX").count()
                        } else {
                            0
                        }
                        + if n > 2 {
                            let ldiag_word = line1.chars().nth(n).unwrap().to_string()
                                + &line2.chars().nth(n - 1).unwrap().to_string()
                                + &line3.chars().nth(n - 2).unwrap().to_string()
                                + &line4.chars().nth(n - 3).unwrap().to_string();

                            ldiag_word.matches("XMAS").count() + ldiag_word.matches("SAMX").count()
                        } else {
                            0
                        }
                })
                .sum::<usize>()
                + line1.matches("XMAS").count()
                + line1.matches("SAMX").count()
                + if cl == lines.clone().count() - 4 {
                    line2.matches("XMAS").count()
                        + line2.matches("SAMX").count()
                        + line3.matches("XMAS").count()
                        + line3.matches("SAMX").count()
                        + line4.matches("XMAS").count()
                        + line4.matches("SAMX").count()
                } else {
                    0
                }
        })
        .sum::<usize>();

    let mas_count = lines
        .clone()
        .zip(lines.clone().skip(1))
        .zip(lines.clone().skip(2))
        .map(|((line1, line2), line3)| {
            line1
                .chars()
                .enumerate()
                .map(|(n, _)| {
                    if n < line1.len() - 2 {
                        let branch_l = line1.chars().nth(n).unwrap().to_string()
                            + &line2.chars().nth(n + 1).unwrap().to_string()
                            + &line3.chars().nth(n + 2).unwrap().to_string();

                        let branch_r = line1.chars().nth(n + 2).unwrap().to_string()
                            + &line2.chars().nth(n + 1).unwrap().to_string()
                            + &line3.chars().nth(n).unwrap().to_string();

                        usize::from(
                            (branch_l == "MAS" || branch_l == "SAM")
                                && (branch_r == "MAS" || branch_r == "SAM"),
                        )
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("DAY 4:\n\tXMAS search: {count}\n\tX-MAS search: {mas_count}");
}

use std::fs;

pub fn day14() {
    let buf = fs::read("src/day14_input.txt").expect("Couldn't read input");
    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let lines = input.lines();

    let mut robots: Vec<((i32, i32), (i32, i32))> = lines
        .map(|l| {
            (
                l.split(' ')
                    .next()
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .split_once(',')
                    .map(|(n, m)| (n.parse::<i32>().unwrap(), m.parse::<i32>().unwrap()))
                    .unwrap(),
                l.split(' ')
                    .last()
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .split_once(',')
                    .map(|(n, m)| (n.parse::<i32>().unwrap(), m.parse::<i32>().unwrap()))
                    .unwrap(),
            )
        })
        .collect();

    let (width, height): (i32, i32) = (101, 103);

    for frame in 0..10000 {
        robots = robots
            .into_iter()
            .map(|((x, y), (dx, dy))| {
                let (nx, ny) = (
                    // Wraps values around
                    // 1st: Reduces to range between -max and max
                    // 2nd: Add max to make positive
                    // 3rd: Convert to usize to remove sign
                    (((x + dx) % width) + width) % width,
                    (((y + dy) % height) + height) % height,
                );

                ((nx, ny), (dx, dy))
            })
            .collect();

        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

        let (mid_x, mid_y) = (width / 2, height / 2);

        for ((x, y), (_, _)) in robots.clone() {
            if x < mid_x && y < mid_y {
                q1 += 1;
            } else if x > mid_x && y < mid_y {
                q2 += 1;
            } else if x < mid_x && y > mid_y {
                q3 += 1;
            } else if x > mid_x && y >= mid_y {
                q4 += 1;
            }
        }

        if q1 > 300 || q2 > 300 || q3 > 300 || q4 > 300 {
            println!("--{frame}--");
            for y in 0..height {
                for x in 0..width {
                    let count = robots
                        .iter()
                        .filter(|((a, b), (_, _))| *a == x && *b == y)
                        .count();
                    match count {
                        0 => print!("."),
                        _ => print!("{count}"),
                    }
                }
                println!();
            }
            println!();
            println!();
        }
    }

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    let (mid_x, mid_y) = (width / 2, height / 2);

    for ((x, y), (_, _)) in robots {
        if x < mid_x && y < mid_y {
            q1 += 1;
        } else if x > mid_x && y < mid_y {
            q2 += 1;
        } else if x < mid_x && y > mid_y {
            q3 += 1;
        } else if x > mid_x && y >= mid_y {
            q4 += 1;
        }
    }

    println!(
        "DAY 14:\n\t{q1} * {q2} * {q3} * {q4} = {}",
        q1 * q2 * q3 * q4
    );
}

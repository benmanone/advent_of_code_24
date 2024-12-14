use std::fs;

pub fn day13() {
    let buf = fs::read("src/day13_input.txt").expect("Couldn't read input");
    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let cost: f64 = input
        .split("\n\n")
        .map(|sys| {
            let ax = sys
                .lines()
                .next()
                .unwrap()
                .split_once("X+")
                .unwrap()
                .1
                .split_once(',')
                .unwrap()
                .0
                .parse::<f64>()
                .unwrap();
            let ay = sys
                .lines()
                .next()
                .unwrap()
                .split_once("Y+")
                .unwrap()
                .1
                .parse::<f64>()
                .unwrap();

            let bx = sys
                .lines()
                .nth(1)
                .unwrap()
                .split_once("X+")
                .unwrap()
                .1
                .split_once(',')
                .unwrap()
                .0
                .parse::<f64>()
                .unwrap();
            let by = sys
                .lines()
                .nth(1)
                .unwrap()
                .split_once("Y+")
                .unwrap()
                .1
                .parse::<f64>()
                .unwrap();

            let px = sys
                .lines()
                .nth(2)
                .unwrap()
                .split_once("X=")
                .unwrap()
                .1
                .split_once(',')
                .unwrap()
                .0
                .parse::<f64>()
                .unwrap()
                + 10000000000000.0;
            let py = sys
                .lines()
                .nth(2)
                .unwrap()
                .split_once("Y=")
                .unwrap()
                .1
                .parse::<f64>()
                .unwrap()
                + 10000000000000.0;

            let det = ax.mul_add(by, -(bx * ay));
            let a = px.mul_add(by, -(py * bx)) / det;
            let b = ax.mul_add(py, -(px * ay)) / det;

            if a.fract() == 0.0 && b.fract() == 0.0 {
                3.0f64.mul_add(a, b)
            } else {
                0.0
            }
        })
        .sum();

    println!("DAY 13:\n\t{cost}");
}

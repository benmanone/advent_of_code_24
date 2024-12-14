use std::fs;

fn search(
    map: &Vec<String>,
    c: char,
    coord: (i32, i32),
    graph: &mut Vec<(i32, i32)>,
    visited: &mut Vec<(i32, i32)>,
) {
    visited.push(coord);
    graph.push(coord);

    for d in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let (nx, ny) = (coord.0 + d.0, coord.1 + d.1);
        if nx >= 0
            && ny >= 0
            && ny < map.len() as i32
            && nx < map.first().unwrap().len() as i32
            && map
                .get(ny as usize)
                .unwrap_or(&" ".to_owned())
                .chars()
                .nth(nx as usize)
                .unwrap_or(' ')
                == c
            && !visited.contains(&(nx, ny))
        {
            search(map, c, (nx, ny), graph, visited);
        }
    }
}

fn perimeter(graph: &Vec<(i32, i32)>) -> usize {
    graph
        .iter()
        .map(|(x, y)| {
            4 - [(0, -1), (-1, 0), (1, 0), (0, 1)]
                .iter()
                .map(|(dx, dy)| graph.contains(&(x + dx, y + dy)))
                .filter(|b| *b)
                .count()
        })
        .sum()
}

fn corners(graph: &Vec<(i32, i32)>) -> usize {
    graph
        .iter()
        .map(|(x, y)| {
            let mut total = 0;

            if !graph.contains(&(*x, y - 1)) && !graph.contains(&(x - 1, *y)) {
                total += 1;
            }
            if !graph.contains(&(*x, y - 1)) && !graph.contains(&(x + 1, *y)) {
                total += 1;
            }
            if !graph.contains(&(*x, y + 1)) && !graph.contains(&(x - 1, *y)) {
                total += 1;
            }
            if !graph.contains(&(*x, y + 1)) && !graph.contains(&(x + 1, *y)) {
                total += 1;
            }
            if graph.contains(&(x + 1, *y))
                && graph.contains(&(*x, y + 1))
                && !graph.contains(&(x + 1, y + 1))
            {
                total += 1;
            }
            if graph.contains(&(x - 1, *y))
                && graph.contains(&(*x, y + 1))
                && !graph.contains(&(x - 1, y + 1))
            {
                total += 1;
            }
            if graph.contains(&(x + 1, *y))
                && graph.contains(&(*x, y - 1))
                && !graph.contains(&(x + 1, y - 1))
            {
                total += 1;
            }
            if graph.contains(&(x - 1, *y))
                && graph.contains(&(*x, y - 1))
                && !graph.contains(&(x - 1, y - 1))
            {
                total += 1;
            }
            total
        })
        .sum()
}

pub fn day12() {
    let buf = fs::read("src/day12_input.txt").expect("Couldn't read input");
    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let mut visited: Vec<(i32, i32)> = vec![];
    let mut graphs: Vec<Vec<(i32, i32)>> = vec![];

    let map = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for (y, l) in input.lines().enumerate() {
        for (x, k) in l.chars().enumerate() {
            if visited.contains(&(x as i32, y as i32)) {
                continue;
            }

            let mut graph: Vec<(i32, i32)> = Vec::new();

            search(&map, k, (x as i32, y as i32), &mut graph, &mut visited);

            graphs.push(graph);
        }
    }

    let perimeter_cost: usize = graphs.iter().map(|g| perimeter(g) * g.len()).sum();
    let side_cost: usize = graphs.iter().map(|g| corners(g) * g.len()).sum();

    println!("DAY 12\n\tPerimeter * Area: {perimeter_cost}\n\tSides * Area: {side_cost}");
}

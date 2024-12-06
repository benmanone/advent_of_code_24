use std::fs;

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

impl Rule {
    fn parse(str: &str) -> Option<Self> {
        let mut parts = str.split('|');

        Some(Self {
            before: parts.nth(0)?.parse::<u32>().ok()?,
            after: parts.last()?.parse::<u32>().ok()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn parse(str: &str) -> Self {
        let parts = str.split(',');

        Self {
            pages: parts.flat_map(str::parse).collect::<Vec<u32>>(),
        }
    }

    fn evaluate(&self, rules: &[Rule]) -> bool {
        rules
            .iter()
            .filter_map(|r| {
                if let (Some(before), Some(after)) = (
                    self.pages.iter().position(|n| n == &r.before),
                    self.pages.iter().position(|n| n == &r.after),
                ) {
                    Some(before < after)
                } else {
                    None
                }
            })
            .all(|b| b)
    }

    fn sorted(&self, rules: &[Rule]) -> Self {
        let mut sorted_pages = self.pages.clone();

        sorted_pages.sort_by(|a, b| {
            if rules.iter().any(|r| r.before == *a && r.after == *b) {
                std::cmp::Ordering::Less
            } else if rules.iter().any(|r| r.before == *b && r.after == *a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        Self {
            pages: sorted_pages,
        }
    }
}

pub fn day5() {
    let buf = fs::read("src/day5_input.txt").expect("Couldn't read input");
    let mut input = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {e}"),
    };
    input.pop();

    let mut sections = input.split("\n\n");

    let rules = sections
        .next()
        .unwrap()
        .split('\n')
        .filter_map(Rule::parse)
        .collect::<Vec<Rule>>();

    let updates = sections
        .clone()
        .last()
        .unwrap()
        .split('\n')
        .map(Update::parse);

    let midpoint_sum = updates
        .clone()
        .filter(|u| u.evaluate(&rules))
        .map(|u| {
            let midpoint = ((u.pages.len() + 1) / 2) - 1;
            *u.pages.get(midpoint).unwrap()
        })
        .sum::<u32>();

    let sorted_midpoint_sum = updates
        .filter(|u| !u.evaluate(&rules))
        .map(|u| u.sorted(&rules))
        .map(|u| {
            let midpoint = ((u.pages.len() + 1) / 2) - 1;
            *u.pages.get(midpoint).unwrap()
        })
        .sum::<u32>();

    println!(
        "DAY 5:\n\tMidpoints: {midpoint_sum:?}\n\tSorted midpoint sum: {sorted_midpoint_sum:?}"
    );
}

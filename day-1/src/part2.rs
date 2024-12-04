use std::collections::HashMap;

pub fn process(input: &str) -> (HashMap<u32, (u32, u32)>, u32) {
    let mut group1 = Vec::<u32>::new();
    let mut group2 = Vec::<u32>::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        group1.push(parts.next().unwrap().parse().unwrap());
        group2.push(parts.next().unwrap().parse().unwrap());
    }

    let mut locations = HashMap::new();
    for left in group1.iter() {
        let index = locations.get_mut(left);
        match index {
            Some((score, accumulated_score)) => {
                // already calculated score
                *accumulated_score += *score;
            }
            None => {
                // not in list of locations, count in group2 and calculate score
                let right = group2.iter().filter(|value| *value == left).count();
                let score = left * right as u32;
                locations.insert(*left, (score, score));
            }
        }
    }

    let score = locations.values().map(|(_, score)| score).sum();
    (locations, score)
}

#[test]
fn parse_file() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let (locations, score) = process(&input);
    println!("Distance: {score}");
}

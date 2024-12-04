pub fn process(input: &str) -> u32 {
    let mut first = Vec::<u32>::new();
    let mut second = Vec::<u32>::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        first.push(parts.next().unwrap().parse().unwrap());
        second.push(parts.next().unwrap().parse().unwrap());
    }

    first.sort();
    second.sort();
    first
        .iter()
        .zip(second.iter())
        .map(|(first, second)| first.abs_diff(*second))
        .sum()
}

#[test]
fn parse_file() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let distance = process(&input);
    println!("Distance: {distance}");
}

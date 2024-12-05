fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let result = day_1::part1::process(divan::black_box(&input));
    divan::black_box_drop(result);
}

#[divan::bench]
fn part2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let result = day_1::part2::process(divan::black_box(&input));
    divan::black_box_drop(result);
}

pub(crate) fn run_part_1() {
    let result = std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .lines()
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    println!("day 1 part 1: {}", result);
}

pub(crate) fn run_part_2() {
    let result = std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .lines()
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    println!("day 1 part 2: {}", result);
}

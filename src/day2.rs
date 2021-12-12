pub(crate) fn run_part_1() {
    let mut horizontal = 0isize;
    let mut vertical = 0isize;

    for command in std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .flat_map(|s| s.parse::<Command>())
    {
        match command {
            Command::Up(units) => vertical -= units,
            Command::Down(units) => vertical += units,
            Command::Forward(units) => horizontal += units,
        };
    }

    println!("day 2 part 1: {}", horizontal * vertical);
}

pub(crate) fn run_part_2() {
    let mut horizontal = 0isize;
    let mut vertical = 0isize;
    let mut aim = 0isize;

    for command in std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .flat_map(|s| s.parse::<Command>())
    {
        match command {
            Command::Up(units) => aim -= units,
            Command::Down(units) => aim += units,
            Command::Forward(units) => {
                horizontal += units;
                vertical += aim * units;
            }
        };
    }

    println!("day 2 part 2: {}", horizontal * vertical);
}

enum Command {
    Up(isize),
    Down(isize),
    Forward(isize),
}

impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction, units) = s.split_once(' ').unwrap();
        let units = units.parse::<isize>().unwrap();

        Ok(match direction {
            "up" => Command::Up(units),
            "down" => Command::Down(units),
            "forward" => Command::Forward(units),
            _ => return Err(()),
        })
    }
}

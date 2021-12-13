pub(crate) fn run_part_1() {
    let mut ones = [0usize; usize::BITS as usize];
    let mut bits = 0;
    let mut numbers = 0usize;

    for line in std::fs::read_to_string("inputs/day3.txt").unwrap().lines() {
        bits = std::cmp::max(bits, line.len());
        numbers += 1;

        let number = usize::from_str_radix(line, 2).unwrap();
        for index in 0..bits {
            if number & (1 << index) != 0 {
                ones[index] += 1;
            }
        }
    }

    let median = numbers / 2;
    let mut gamma = 0usize;
    let mut epsilon = 0usize;

    for index in 0..bits {
        if ones[index] > median {
            gamma |= 1 << index;
        } else {
            epsilon |= 1 << index;
        }
    }

    println!("day 3 part 1: {}", gamma * epsilon);
}

pub(crate) fn run_part_2() {
    let mut bits = 0;
    let numbers = std::fs::read_to_string("inputs/day3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let number = usize::from_str_radix(line, 2).unwrap();
            let len = line.len() as u8;

            bits = std::cmp::max(bits, len);
            number
        })
        .collect::<Vec<_>>();

    fn find_number(numbers: &[usize], bits: u8, frequent: bool) -> Option<usize> {
        let mut generations = vec![1u8; numbers.len()];
        for bit in (0..bits).rev() {
            let mut len = 0usize;
            let mut ones = 0usize;
            let current_gen = bits - bit;
            for (index, number) in numbers.iter().enumerate() {
                if generations[index] == current_gen {
                    len += 1;
                    if number & (1 << bit) != 0 {
                        ones += 1;
                    }
                }
            }

            let pattern = if len <= ones * 2 {
                frequent as usize
            } else {
                !frequent as usize
            };
            let mut results = 0usize;
            let mut result = None;
            for (index, number) in numbers.iter().enumerate() {
                if generations[index] == current_gen {
                    if pattern == ((*number >> bit) & 1) {
                        generations[index] = current_gen + 1;

                        results += 1;
                        result = Some(*number);
                    }
                }
            }

            if results == 1 {
                return result;
            }
        }

        None
    }

    let oxygen = find_number(&numbers, bits, true).unwrap();
    let co2 = find_number(&numbers, bits, false).unwrap();

    println!("day 3 part 2: {}", oxygen * co2);
}

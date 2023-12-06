fn main() {
    let input = include_str!("./input2.txt").to_string();
    let output = part2(input);
    dbg!(output);
}


fn parse_input(input: String) -> Vec<(u64, u64)> {
    let parts = input
        .lines()
        .map(|line| {
            line.split(':')
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        }).collect::<Vec<Vec<u64>>>();

    parts[0].iter().zip(parts[1].iter()).map(|(a, b)| (*a, *b)).collect()
}

fn part2(input: String) -> u64 {
    parse_input(input)
        .iter()
        .fold(1, |acc, (time, distance)| {
            let mut possibilities = 0u64;
            let middle = *time / 2;

            for i in 1..=middle {
                let d = i * (time - i);
                if d > *distance {
                    possibilities += 1;
                }
            }
            possibilities *= 2;
            if time % 2 == 0 {
                possibilities -= 1;
            }

            acc * possibilities
        })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_2() {
        let input = include_str!("./example2.txt").to_string();
        let want = include_str!("./answer2.txt").parse::<u64>().expect("could not parse answer2.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }
}
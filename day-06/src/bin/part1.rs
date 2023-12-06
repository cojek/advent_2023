fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input.to_string());
    dbg!(output);
}

fn parse_input(input: String) -> Vec<(u32, u32)> {
    let parts = input
        .lines()
        .map(|line| {
            line.split(':')
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();

    parts[0].iter().zip(parts[1].iter()).map(|(a, b)| (*a, *b)).collect()
}

fn part1(input: String) -> u32 {
    parse_input(input)
        .iter()
        .fold(1, |acc, (time, distance)| {
            let mut possibilities = 0u32;
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
    fn test_1() {
        let input = include_str!("./example1.txt").to_string();
        let want = include_str!("./answer1.txt").parse::<u32>().expect("could not parse answer1.txt");

        let got = part1(input);
        assert_eq!(got, want);
    }
}

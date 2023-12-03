fn main() {
    let input = include_str!("./input1.txt").to_string();
    let output = part2(input);
    dbg!(output);
}

fn find_minimum_required_cubes(game_description: &str) -> (u32, u32, u32) {
    let mut max = (0, 0, 0);

    for set in game_description.split(';') {
        for cubes in set.split(',') {
            let parts = cubes.trim().split(' ').collect::<Vec<&str>>();

            let count = parts[0].parse::<u32>().expect("could not parse cube count");
            match parts[1] {
                "red" => max.0 = std::cmp::max(max.0, count),
                "green" => max.1 = std::cmp::max(max.1, count),
                "blue" => max.2 = std::cmp::max(max.2, count),
                _ => panic!("invalid cube type {}", parts[1])
            }
        }
    }

    max
}

fn part2(input: String) -> u32 {
    input
        .lines()
        .fold(0, |acc, l| {
            let parts = l.split(':').collect::<Vec<&str>>();
            let max = find_minimum_required_cubes(parts[1]);

            acc + (max.0 * max.1 * max.2)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_2() {
        let input = include_str!("./example2.txt").to_string();
        let want = include_str!("./answer2.txt").parse::<u32>().expect("could not parse answer2.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }
}
use std::fmt::format;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input.to_string());
    dbg!(output);
}

fn part1(input: String) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| {
            let parts: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1].split('|').collect();
            let winning = parts[0]
                .trim()
                .split_whitespace()
                .map(|x| {
                    x.parse::<u32>().expect(format!("could not parse winning number: {}", x).as_str())
                }).collect::<Vec<u32>>();
            let scratched = parts[1]
                .trim()
                .split_whitespace()
                .map(|x| {
                    x.parse::<u32>().expect("could not parse scratched number")
                })
                .fold(0, |acc, number| {
                    if winning.contains(&number) {
                        acc + 1
                    } else {
                        acc
                    }
                });

            acc + if scratched != 0 { 2u32.pow(scratched-1) } else { 0 }
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

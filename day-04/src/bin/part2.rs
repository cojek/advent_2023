fn main() {
    let input = include_str!("./input1.txt").to_string();
    let output = part2(input);
    dbg!(output);
}

fn part2(input: String) -> u32 {
    let mut cards = vec![1;input.lines().count()];

    input
        .lines()
        .fold(0, |acc, line| {
            let mut parts: Vec<&str> = line.split(':').collect::<Vec<&str>>();
            let id = parts[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<usize>().expect("could not parse card id");

            parts = parts[1].split('|').collect();
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

            let card_count = cards[id-1];
            if scratched > 0 {
                for i in id+1..=id+scratched {
                    cards[i-1] += card_count;
                }
            }


            acc + card_count
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
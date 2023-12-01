fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .fold(0, |acc, line| {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;

            for c in line.chars() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = Some(c);
                    }
                    last = Some(c);
                }
            }

            acc + match first {
                None => 0,
                Some(c) =>
                    format!("{}{}", c, last.unwrap()).parse::<i32>().unwrap_or_else(|_| panic!("could not parse: {}{}", c, last.unwrap()))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("./example1.txt");
        let want = include_str!("./answer1.txt").parse::<i32>().expect("could not parse answer1.txt");

        let got = part1(input);
        assert_eq!(got, want);
    }
}
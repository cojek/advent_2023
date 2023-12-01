fn main() {
    let input = include_str!("./input1.txt").to_string();
    let output = part2(input);
    dbg!(output);
}

fn append_char(buf: &mut [char; 5], c: char) {
    for i in 1..=4 {
        buf[i - 1] = buf[i];
    }
    buf[4] = c;
}

fn check_for_written_digit(buf: [char; 5]) -> Option<u32> {
    let digits: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    for (idx, val) in digits.iter().enumerate() {
        if buf[5 - val.len()..5].iter().collect::<String>().eq(val) {
            return Some((idx + 1) as u32);
        }
    }
    None
}

fn part2(input: String) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| {
            let mut first: u32 = 0;
            let mut last: u32 = 0;

            let mut buffer = ['0'; 5];
            for c in line.chars() {
                append_char(&mut buffer, c);

                // Check digits
                let mut i: u32 = 0;
                if let Some(u) = c.to_digit(10) {
                    i = u;
                } else {
                    // Check written out
                    match check_for_written_digit(buffer) {
                        None => {}
                        Some(u) => i = u
                    }
                }

                if i != 0 {
                    if first == 0 {
                        first = i;
                    }
                    last = i;
                }
            }

            acc + (10 * first + last)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_char() {
        let mut input = ['1', '2', '3', '4', '5'];
        append_char(&mut input, '6');
        append_char(&mut input, '7');
        let want = ['3', '4', '5', '6', '7'];

        assert_eq!(input, want);
    }

    #[test]
    fn test_example_1() {
        let input = include_str!("./example1.txt").to_string();
        let want = include_str!("./answer1.txt").parse::<u32>().expect("could not parse answer1.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("./example2.txt").to_string();
        let want = include_str!("./answer2.txt").parse::<u32>().expect("could not parse answer2.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_1() {
        let input = "1".to_string();
        let want: u32 = 11;

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_2() {
        let input = "12".to_string();
        let want: u32 = 12;

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_3() {
        let input = "one2".to_string();
        let want: u32 = 12;

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_4() {
        let input = "onetwo".to_string();
        let want: u32 = 12;

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_5() {
        let input = "1two".to_string();
        let want: u32 = 12;

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_6() {
        let input = "1two\noneight".to_string();
        let want: u32 = 30;

        let got = part2(input);
        assert_eq!(got, want);
    }
}
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn append_char(buf: &mut [char; 5], c: char) {
    for i in 1..=4 {
        buf[i - 1] = buf[i];
    }
    buf[4] = c;
}

fn check_for_written_digit(buf: [char; 5]) -> Option<char> {
    let mut out = None;
    if buf[2..5].iter().collect::<String>() == "one" {
        out = Some('1');
    }
    if buf[2..5].iter().collect::<String>() == "two" {
        out = Some('2');
    }
    if buf[0..5].iter().collect::<String>() == "three" {
        out = Some('3');
    }
    if buf[1..5].iter().collect::<String>() == "four" {
        out = Some('4');
    }
    if buf[1..5].iter().collect::<String>() == "five" {
        out = Some('5');
    }
    if buf[2..5].iter().collect::<String>() == "six" {
        out = Some('6');
    }
    if buf[0..5].iter().collect::<String>() == "seven" {
        out = Some('7');
    }
    if buf[0..5].iter().collect::<String>() == "eight" {
        out = Some('8');
    }
    if buf[1..5].iter().collect::<String>() == "nine" {
        out = Some('9');
    }

    out
}

fn part2(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .map(|s| s.trim())
        .fold(0, |acc, line| {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;

            let mut buffer = ['0'; 5];
            for c in line.chars() {
                let mut c = c;

                // Check written
                append_char(&mut buffer, c);
                match check_for_written_digit(buffer) {
                    None => {}
                    Some(d) => c = d
                }


                // Check digits
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = Some(c);
                    }
                    last = Some(c);
                }
            }


            let out = match first {
                None => 0,
                Some(c) => {
                    format!("{}{}", c, last.unwrap()).parse::<i32>().unwrap_or_else(|_| panic!("could not parse: {}{}", c, last.unwrap()))
                }
            };

            acc + out
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
    fn test_1() {
        let input = include_str!("./example1.txt");
        let want = include_str!("./answer1.txt").parse::<i32>().expect("could not parse answer1.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_2() {
        let input = include_str!("./example2.txt");
        let want = include_str!("./answer2.txt").parse::<i32>().expect("could not parse answer2.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }
}
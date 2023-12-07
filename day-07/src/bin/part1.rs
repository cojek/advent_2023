use std::cmp;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input.to_string());
    dbg!(output);
}

#[derive(PartialEq, Debug)]
enum Hand {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    Pair,
    High,
}

fn char_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("unknown character {}", c)
    }
}

impl Hand {
    fn from_str(input: &str) -> Self {
        let mut count = [0usize; 15];
        for i in input.chars() {
            let v = char_value(i) as usize;
            count[v] += 1;
        }

        let mut largest_idx = 0usize;
        let mut second_idx = 0usize;
        for (idx, i) in count.iter().enumerate() {
            if *i >= count[largest_idx] {
                second_idx = largest_idx;
                largest_idx = idx;
            } else if *i >= count[second_idx] {
                second_idx = idx;
            }
        }

        if count[largest_idx] == 5 {
            Hand::Five
        } else if count[largest_idx] == 4 {
            Hand::Four
        } else if count[largest_idx] == 3 {
            if count[second_idx] == 2 {
                Hand::Full
            } else {
                Hand::Three
            }
        } else if count[largest_idx] == 2 {
            if count[second_idx] == 2 {
                Hand::TwoPair
            } else {
                Hand::Pair
            }
        } else {
            Hand::High
        }
    }

    fn order(&self) -> u32 {
        match self {
            Hand::Five => { 6 }
            Hand::Four => { 5 }
            Hand::Full => { 4 }
            Hand::Three => { 3 }
            Hand::TwoPair => { 2 }
            Hand::Pair => { 1 }
            Hand::High => { 0 }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.order() < other.order() {
            return Some(Ordering::Less);
        }
        if self.order() > other.order() {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }
}

fn parse_input(input: String) -> Vec<((Hand, String), u32)> {
    input
        .lines()
        .map(|x| {
            let parts = x.split_whitespace().collect::<Vec<&str>>();
            (parts[0], parts[1])
        })
        .map(|(h, v)| {
            ((Hand::from_str(h), h.to_string()), v.parse::<u32>().expect("could not parse value"))
        })
        .collect()
}

fn order_func((hand_a, string_a): &(Hand, String), (hand_b, string_b): &(Hand, String)) -> Ordering {
    let val = hand_a.partial_cmp(hand_b).unwrap();
    if val != Ordering::Equal {
        return val;
    }

    for i in 0..string_a.len() {
        let a_v = char_value(string_a.chars().nth(i).unwrap());
        let b_v = char_value(string_b.chars().nth(i).unwrap());
        if a_v < b_v {
            return Ordering::Less;
        }
        if a_v > b_v {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

fn part1(input: String) -> u32 {
    let mut values = parse_input(input);
    values.sort_by(|(hand_a, _), (hand_b, _)| {
        order_func(hand_a, hand_b)
    });
    let mut out = 0u32;
    for (idx, (h, v)) in values.iter().enumerate() {
        out += (idx as u32 + 1) * v;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        let cases = vec!(
            ("22222", Hand::Five),
            ("22223", Hand::Four),
            ("22233", Hand::Full),
            ("22234", Hand::Three),
            ("22334", Hand::TwoPair),
            ("22345", Hand::Pair),
            ("23456", Hand::High),
        );
        for (input, want) in cases {
            let got = Hand::from_str(input);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_ordering() {
        let cases = vec!(
            ("33332", "2AAAA", Ordering::Greater),
            ("77888", "77788", Ordering::Greater)
        );
        for (hand_a, hand_b, want) in cases {
            let a = (Hand::from_str(hand_a), hand_a.to_string());
            let b = (Hand::from_str(hand_b), hand_b.to_string());
            assert_eq!(order_func(&a, &b), want)
        }
    }

    #[test]
    fn test_1() {
        let input = include_str!("./example1.txt").to_string();
        let want = include_str!("./answer1.txt").parse::<u32>().expect("could not parse answer1.txt");

        let got = part1(input);
        assert_eq!(got, want);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("./input1.txt").to_string();
        let want = 250602641u32;

        let got = part1(input);
        assert_eq!(got, want);
    }
}

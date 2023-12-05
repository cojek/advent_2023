fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input.to_string());
    dbg!(output);
}

fn translate(values: Vec<u64>, ranges: &Vec<(u64, u64, u64)>) -> Vec<u64> {
    let mut mapped = vec![];
    for v in values {
        let mut new_v = v;
        for (dst, src, length) in ranges {
            if v >= *src && v < *src + length {
                new_v = v + dst - src;
            }
        }
        mapped.push(new_v);
    }

    mapped
}

fn read_input(input: String) -> (Vec<u64>, Vec<(String, Vec<(u64, u64, u64)>)>) {
    let mut values = vec![];
    let mut iter = input.split_whitespace().peekable();

    // read values
    iter.next(); // skip "seeds:"
    while let Some(v) = iter.peek() {
        if let Ok(v) = (*v).parse::<u64>() {
            values.push(v);
            iter.next();
        } else {
            break;
        }
    }

    // read mappings
    let mut mapping = vec![];

    while let Some(v) = iter.next() {
        let name = v.to_string();
        iter.next(); // skip "map:"
        let mut map = vec![];
        while let Some(v) = iter.peek() {
            if let Err(_) = (*v).parse::<u64>() {
                break; // Reached next map
            }

            let mut t = (0u64, 0u64, 0u64);
            for i in 0..3 {
                let v = iter.next().unwrap();
                t.0 = t.1;
                t.1 = t.2;
                t.2 = (*v).parse::<u64>().unwrap();
            }

            map.push(t);
        }

        mapping.push((name, map))
    }
    (values, mapping)
}

fn part1(input: String) -> u64 {
    let (values, mapping) = read_input(input);
    mapping
        .iter()
        .fold(values, |acc, (name, map)| {
            translate(acc, map)
        })
        .iter().min().expect("could not get minimal number").clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("./example1.txt").to_string();
        let want = include_str!("./answer1.txt").parse::<u64>().expect("could not parse answer1.txt");

        let got = part1(input);
        assert_eq!(got, want);
    }
}

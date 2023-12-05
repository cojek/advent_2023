fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input.to_string());
    dbg!(output);
}

fn translate_ranges(values: Vec<(u64, u64)>, ranges: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut out = vec![];
    for v in values {
        out.push(translate_range(v, ranges))
    }

    out.iter().flatten().map(|x| *x).collect()
}

fn translate_range(range: (u64, u64), mappings: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let (mut start, mut end) = range;

    let mut translations = vec![];
    let mut found = false;
    for (dst, src, length) in mappings {
        if end < *src || start >= (*src + *length) {
            continue; // out of range
        }

        if start < *src {
            translations.push(translate_range((start, *src - 1), mappings));
            start = *src;
        }

        if end >= *src + length {
            translations.push(translate_range((*src + length, end), mappings));
            end = *src + length - 1;
        }

        found = true;
        translations.push(vec![(start + dst - src, end + dst - src)]);
        break;
    }

    if !found {
        translations.push(vec![(start, end)]);
    }
    translations.iter().flatten().map(|x| *x).collect()
}


fn read_input(input: String) -> (Vec<(u64, u64)>, Vec<(String, Vec<(u64, u64, u64)>)>) {
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

    let values = values.iter()
        .step_by(2)
        .zip(values.iter().skip(1).step_by(2))
        .map(|(start, count)| (*start, *start + count - 1))
        .collect();

    // read mappings
    let mut mapping = vec![];

    while let Some(v) = iter.next() {
        let name = v.to_string();
        iter.next(); // skip "map:"
        let mut vals = vec![];
        while let Some(v) = iter.peek() {
            match (*v).parse::<u64>() {
                Ok(v) => { vals.push(v) }
                Err(_) => { break; /* Reached next map */ }
            }
            iter.next();
        }

        let map = vals.iter()
            .step_by(3)
            .zip(vals.iter().skip(1).step_by(3))
            .zip(vals.iter().skip(2).step_by(3))
            .map(|x| (*x.0.0, *x.0.1, *x.1))
            .collect::<Vec<(u64, u64, u64)>>();

        mapping.push((name, map))
    }
    (values, mapping)
}

fn part2(input: String) -> u64 {
    let (values, mapping) = read_input(input);
    mapping
        .iter()
        .fold(values, |acc, (name, map)| {
            translate_ranges(acc, map)
        })
        .iter()
        .map(|(start, end)| start).min().expect("could not get minimal number").clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let input = include_str!("./example2.txt").to_string();
        let want = include_str!("./answer2.txt").parse::<u64>().expect("could not parse answer2.txt");

        let got = part2(input);
        assert_eq!(got, want);
    }
}

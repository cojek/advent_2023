fn main() {
    let input = include_str!("./input1.txt").to_string();
    let output = part2(input);
    dbg!(output);
}

type Map = Vec<Vec<char>>;

fn build_matrix(input: String) -> Map {
    let mut out = vec![];

    for l in input.lines() {
        let mut line: Vec<char> = vec![];
        for c in l.chars() {
            line.push(c)
        }

        out.push(line);
    }

    out
}

fn get_number(map: &Map, x: i32, y: i32) -> u32 {
    // Check boundaries
    if !is_digit(map, x, y) {
        return 0;
    }

    let y = y as usize;
    let mut left = x as usize;
    let mut right = x as usize;
    while left > 0 {
        if !map[y][left - 1].is_ascii_digit() {
            break;
        }
        left -= 1;
    }
    while right < map[y].len() - 1 {
        if !map[y][right + 1].is_ascii_digit() {
            break;
        }
        right += 1;
    }

    let mut out = 0;
    for i in map[y][left..right + 1].iter() {
        out = 10 * out + i.to_digit(10).unwrap();
    }

    out
}

fn is_digit(map: &Map, x: i32, y: i32) -> bool {
    if y < 0 || y >= (map.len() as i32) {
        return false;
    }
    if x < 0 || x >= (map[0].len() as i32) {
        return false;
    }
    map[y as usize][x as usize].is_ascii_digit()
}

fn check_gear(map: &Map, x: i32, y: i32) -> u32 {
    let mut number_count = 0;
    let mut ratio = 1;

    // Check above
    if is_digit(map, x, y - 1) {
        ratio *= get_number(map, x, y - 1);
        number_count += 1;
    } else if is_digit(map, x - 1, y - 1) && is_digit(map, x + 1, y - 1) {
        ratio *= get_number(map, x - 1, y - 1) * get_number(map, x + 1, y - 1);
        number_count += 2;
    } else if is_digit(map, x - 1, y - 1) {
        ratio *= get_number(map, x - 1, y - 1);
        number_count += 1;
    } else if is_digit(map, x + 1, y - 1) {
        ratio *= get_number(map, x + 1, y - 1);
        number_count += 1;
    }

    // Check left / right
    if is_digit(map, x - 1, y) {
        ratio *= get_number(map, x - 1, y);
        number_count += 1;
    }
    if is_digit(map, x + 1, y) {
        ratio *= get_number(map, x + 1, y);
        number_count += 1;
    }

    // Check below
    if is_digit(map, x, y + 1) {
        ratio *= get_number(map, x, y + 1);
        number_count += 1;
    } else if is_digit(map, x - 1, y + 1) && is_digit(map, x + 1, y + 1) {
        ratio *= get_number(map, x - 1, y + 1) * get_number(map, x + 1, y + 1);
        number_count += 2;
    } else if is_digit(map, x - 1, y + 1) {
        ratio *= get_number(map, x - 1, y + 1);
        number_count += 1;
    } else if is_digit(map, x + 1, y + 1) {
        ratio *= get_number(map, x + 1, y + 1);
        number_count += 1;
    }

    if number_count == 2 {
        ratio
    } else {
        0
    }
}

fn part2(input: String) -> u32 {
    let mut map = build_matrix(input);
    let mut out = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c == '*' {
                out += check_gear(&map, x as i32, y as i32);
            }
        }
    }
    out
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
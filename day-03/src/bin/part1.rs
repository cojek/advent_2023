fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input.to_string());
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

fn get_number(map: &mut Map, x: i32, y: i32) -> u32 {
    // Check boundaries
    if y < 0 || y >= (map.len() as i32) {
        return 0;
    }
    if x < 0 || x >= (map[0].len() as i32) {
        return 0;
    }
    if !map[y as usize][x as usize].is_ascii_digit() {
        return 0;
    }

    let y = y as usize;
    let mut left = x as usize;
    let mut right = x as usize;
    while left > 0 {
        if !map[y][left - 1].is_ascii_digit() {
            break
        }
        left -= 1;
    }
    while right < map[y].len() - 1 {
        if !map[y][right + 1].is_ascii_digit() {
            break
        }
        right += 1;
    }

    let mut out = 0;
    for i in map[y][left..right + 1].iter() {
        out = 10 * out + i.to_digit(10).unwrap();
    }

    // remove number to prevent counting twice
    for x in left..=right {
        map[y][x] = '.'
    }

    out
}

fn count_neighbours(map: &mut Map, x: i32, y: i32) -> u32 {
    get_number(map, x - 1, y - 1) +
        get_number(map, x, y - 1) +
        get_number(map, x + 1, y - 1) +
        get_number(map, x - 1, y) +
        get_number(map, x + 1, y) +
        get_number(map, x - 1, y + 1) +
        get_number(map, x, y + 1) +
        get_number(map, x + 1, y + 1)
}

fn part1(input: String) -> u32 {
    let mut map = build_matrix(input);
    let mut out = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c.is_ascii_digit() || c == '.' {
                continue;
            }
            out += count_neighbours(&mut map, x as i32, y as i32)
        }
    }
    out
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

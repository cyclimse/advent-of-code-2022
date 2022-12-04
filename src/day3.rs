#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| {
            let line: Vec<char> = line.chars().collect();
            let middle = line.len() / 2;
            let left = &line[..middle];
            let right = &line[middle..];
            let mut res: i32 = 0;
            for c in left {
                if right.contains(&c) {
                    if c.is_lowercase() {
                        res = (*c as i32) - ('a' as i32) + 1;
                    } else {
                        res = (*c as i32) - ('A' as i32) + 27;
                    }
                }
            }
            res
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut res = 0;
    for i in (0..lines.len()).step_by(3) {
        let (one, two, three): (Vec<char>, Vec<char>, Vec<char>) = (
            lines[i].chars().collect(),
            lines[i + 1].chars().collect(),
            lines[i + 2].chars().collect(),
        );
        for c in one {
            if two.contains(&c) && three.contains(&c) {
                if c.is_lowercase() {
                    res += (c as i32) - ('a' as i32) + 1;
                } else {
                    res += (c as i32) - ('A' as i32) + 27;
                }
                break;
            }
        }
    }
    res
}

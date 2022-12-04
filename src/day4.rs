fn fully_contains(a: i32, b: i32, c: i32, d: i32) -> bool {
    (a <= c) && (b >= d)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n")
        .map(|elf_pair| {
            let splt: Vec<_> = elf_pair.split(',').collect();
            let (one, two): (Vec<_>, Vec<_>) =
                (splt[0].split("-").collect(), splt[1].split("-").collect());
            let (a, b, c, d) = (
                one[0].parse::<i32>().unwrap(),
                one[1].parse::<i32>().unwrap(),
                two[0].parse::<i32>().unwrap(),
                two[1].parse::<i32>().unwrap(),
            );
            (fully_contains(a, b, c, d) || fully_contains(c, d, a, b)) as i32
        })
        .sum()
}

fn does_not_overlap(a: i32, b: i32, c: i32, d: i32) -> bool {
    (b < c) || (d < a)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .split("\n")
        .map(|elf_pair| {
            let splt: Vec<_> = elf_pair.split(',').collect();
            let (one, two): (Vec<_>, Vec<_>) =
                (splt[0].split("-").collect(), splt[1].split("-").collect());
            let (a, b, c, d) = (
                one[0].parse::<i32>().unwrap(),
                one[1].parse::<i32>().unwrap(),
                two[0].parse::<i32>().unwrap(),
                two[1].parse::<i32>().unwrap(),
            );
            !does_not_overlap(a, b, c, d) as i32
        })
        .sum()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|str_for_elf| str_for_elf.split('\n'))
        .map(|arr| {
            arr.map(|entry| entry.parse::<i32>().expect("should be a number"))
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut arr = input
        .split("\n\n")
        .map(|str_for_elf| str_for_elf.split('\n'))
        .map(|arr| {
            arr.map(|entry| entry.parse::<i32>().expect("should be a number"))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    arr.sort();
    arr.pop().unwrap() + arr.pop().unwrap() + arr.pop().unwrap()
}

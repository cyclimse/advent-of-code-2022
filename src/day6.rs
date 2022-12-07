use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    let mut uniques: HashSet<char> = std::collections::HashSet::new();
    for i in 0..chars.len() - 14 {
        for c in &chars[i..(i + 14)] {
            uniques.insert(*c);
        }
        if uniques.len() == 14 {
            return i + 14;
        }
        uniques.clear();
    }
    0
}

// #[aoc(day6, part2)]
// pub fn part2(input: &str) -> i32 {
//     let mut arr = input
//         .split("\n\n")
//         .map(|str_for_elf| str_for_elf.split('\n'))
//         .map(|arr| {
//             arr.map(|entry| entry.parse::<i32>().expect("should be a number"))
//                 .sum::<i32>()
//         })
//         .collect::<Vec<i32>>();
//     arr.sort();
//     arr.pop().unwrap() + arr.pop().unwrap() + arr.pop().unwrap()
// }

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let forest: Vec<Vec<u32>> = input
        .split("\n")
        .map(|row| row.chars())
        .map(|arr| {
            arr.map(|entry| entry.to_digit(10).expect("Should be a number"))
                .collect()
        })
        .collect();
    let n = forest.len();
    let p = forest[0].len();
    let mut count_visible = (n * p) - ((n - 2) * (p - 2));
    for i in 1..(n - 1) {
        for j in 1..(p - 1) {
            let tree = forest[i][j];
            if tree > *forest[i][0..j].iter().max().unwrap_or(&0) {
                count_visible += 1;
                continue;
            }
            if tree > *forest[i][(j + 1)..p].iter().max().unwrap_or(&0) {
                count_visible += 1;
                continue;
            }
            if tree > (0..i).map(|k| forest[k][j]).max().unwrap_or(0) {
                count_visible += 1;
                continue;
            }
            if tree > ((i + 1)..n).map(|k| forest[k][j]).max().unwrap_or(0) {
                count_visible += 1;
                continue;
            }
        }
    }
    count_visible
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let forest: Vec<Vec<u32>> = input
        .split("\n")
        .map(|row| row.chars())
        .map(|arr| {
            arr.map(|entry| entry.to_digit(10).expect("Should be a number"))
                .collect()
        })
        .collect();
    let n = forest.len();
    let p = forest[0].len();
    let mut max_scenic_score = 0;
    for i in 1..(n - 1) {
        for j in 1..(p - 1) {
            let tree = forest[i][j];
            let mut scene_score = 1;
            let iter = forest[i][0..j].iter().rev();
            let mut count = iter.clone().take_while(|t| tree > **t).count();
            if iter.count() != count {
                count += 1;
            }
            scene_score *= count;
            let iter = forest[i][(j + 1)..p].iter();
            let mut count = iter.clone().take_while(|t| tree > **t).count();
            if iter.count() != count {
                count += 1;
            }
            scene_score *= count;
            let iter = (0..i).rev().map(|k| forest[k][j]);
            let mut count = iter.clone().take_while(|t| tree > *t).count();
            if iter.count() != count {
                count += 1;
            }
            scene_score *= count;
            let iter = ((i + 1)..n).map(|k| forest[k][j]);
            let mut count = iter.clone().take_while(|t| tree > *t).count();
            if iter.count() != count {
                count += 1;
            }
            scene_score *= count;
            if scene_score > max_scenic_score {
                max_scenic_score = scene_score;
            }
        }
    }
    max_scenic_score
}

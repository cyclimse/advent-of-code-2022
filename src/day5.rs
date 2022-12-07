use regex::Regex;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let input: Vec<_> = input.split("\n\n").collect();
    let (boxes, instr) = (input[0], input[1]);
    // Parse crates
    let boxes: Vec<_> = boxes.split("\n").collect();
    let mut stacks = vec![Vec::<char>::new(); boxes[0].len() / 2];

    for i in 0..(boxes.len() - 1) {
        let line: Vec<char> = boxes[i].chars().collect();
        let mut k: usize = 0;
        for j in (1..line.len()).step_by(4) {
            if line[j] != ' ' {
                stacks[k].push(line[j]);
            }
            k += 1;
        }
    }
    for i in 0..stacks.len() {
        stacks[i].reverse();
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for cap in re.captures_iter(instr) {
        // println!("{:?}", stacks);
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        let n = cap[1].parse::<usize>().unwrap();
        // println!("move {} from {} to {}", n, from, to);
        for _ in 0..n {
            let bo = stacks[from].pop().expect("should have");
            // println!("{}", bo);
            stacks[to].push(bo);
        }
        // println!("from: {:?}", stacks[from]);
        // println!("to: {:?}", stacks[to]);
    }
    // println!("{:?}", stacks);
    stacks
        .iter_mut()
        .map(|v| v.pop())
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let input: Vec<_> = input.split("\n\n").collect();
    let (boxes, instr) = (input[0], input[1]);
    // Parse crates
    let boxes: Vec<_> = boxes.split("\n").collect();
    let mut stacks = vec![Vec::<char>::new(); boxes[0].len() / 2];

    for i in 0..(boxes.len() - 1) {
        let line: Vec<char> = boxes[i].chars().collect();
        let mut k: usize = 0;
        for j in (1..line.len()).step_by(4) {
            if line[j] != ' ' {
                stacks[k].push(line[j]);
            }
            k += 1;
        }
    }
    for i in 0..stacks.len() {
        stacks[i].reverse();
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for cap in re.captures_iter(instr) {
        // println!("{:?}", stacks);
        let from = cap[2].parse::<usize>().unwrap() - 1;
        let to = cap[3].parse::<usize>().unwrap() - 1;
        let n = cap[1].parse::<usize>().unwrap();
        // println!("move {} from {} to {}", n, from, to);
        let p = stacks[from].len();
        let mut q = stacks[from][p - n..p].to_vec();
        stacks[to].append(&mut q);
        for _ in 0..n {
            stacks[from].pop().expect("should have");
        }
        // println!("from: {:?}", stacks[from]);
        // println!("to: {:?}", stacks[to]);
    }
    // println!("{:?}", stacks);
    stacks
        .iter_mut()
        .map(|v| v.pop())
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect()
}

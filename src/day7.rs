use std::collections::HashMap;

const MAX_SIZE: i32 = 100000;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> i32 {
    // The int is the size of the top level files
    let mut tree = HashMap::<String, i32>::new();

    let mut current_directory = "".to_owned();
    let mut iter = input.split('\n').peekable();
    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.starts_with('$') {
            let splt: Vec<_> = line.split(' ').collect();
            match splt[1] {
                "cd" => match splt[2] {
                    "." => (),
                    ".." => {
                        let arr: Vec<_> = current_directory.split('/').collect();
                        current_directory = arr[..arr.len() - 2].join("/");
                        if !current_directory.ends_with('/') {
                            current_directory.push('/');
                        }
                    }
                    dir => {
                        current_directory.push_str(dir);
                        if !current_directory.ends_with('/') {
                            current_directory.push('/');
                        }
                    }
                },
                "ls" => {
                    let mut size = 0;
                    let mut line = iter.next();
                    while line.is_some() {
                        let splt: Vec<_> = line.unwrap().split(' ').collect();
                        // println!("{:?}", splt);
                        match splt[0].parse::<i32>() {
                            Ok(val) => size += val,
                            _ => (),
                        }
                        if iter.peek().is_none() || iter.peek().unwrap().starts_with("$") {
                            tree.insert(current_directory.clone(), size);
                            break;
                        }
                        line = iter.next();
                    }
                }
                _ => {
                    println!("unkown command {}", splt[1]);
                }
            }
        }
    }
    // println!("{:?}", tree);

    let mut answer = 0;
    for tpls in tree.iter() {
        let (dir, size) = tpls;
        let mut dir_size = *size;
        for tpls in tree.iter() {
            let (child, child_size) = tpls;
            if child != dir && child.starts_with(dir) {
                dir_size += child_size;
            }
        }
        if dir_size <= MAX_SIZE {
            answer += dir_size;
        }
    }
    answer
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
    // The int is the size of the top level files
    let mut tree = HashMap::<String, i32>::new();

    let mut current_directory = "".to_owned();
    let mut iter = input.split('\n').peekable();
    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line.starts_with('$') {
            let splt: Vec<_> = line.split(' ').collect();
            match splt[1] {
                "cd" => match splt[2] {
                    "." => (),
                    ".." => {
                        let arr: Vec<_> = current_directory.split('/').collect();
                        current_directory = arr[..arr.len() - 2].join("/");
                        if !current_directory.ends_with('/') {
                            current_directory.push('/');
                        }
                    }
                    dir => {
                        current_directory.push_str(dir);
                        if !current_directory.ends_with('/') {
                            current_directory.push('/');
                        }
                    }
                },
                "ls" => {
                    let mut size = 0;
                    let mut line = iter.next();
                    while line.is_some() {
                        let splt: Vec<_> = line.unwrap().split(' ').collect();
                        // println!("{:?}", splt);
                        match splt[0].parse::<i32>() {
                            Ok(val) => size += val,
                            _ => (),
                        }
                        if iter.peek().is_none() || iter.peek().unwrap().starts_with("$") {
                            tree.insert(current_directory.clone(), size);
                            break;
                        }
                        line = iter.next();
                    }
                }
                _ => {
                    println!("unkown command {}", splt[1]);
                }
            }
        }
    }
    // println!("{:?}", tree);
    let mut total_tree = tree.clone();

    for tpls in tree.iter() {
        let (dir, size) = tpls;
        let mut dir_size = *size;
        for tpls in tree.iter() {
            let (child, child_size) = tpls;
            if child != dir && child.starts_with(dir) {
                dir_size += child_size;
            }
        }
        total_tree.insert(dir.to_owned(), dir_size);
    }

    const DISK_SIZE: i32 = 70000000;
    const UPDATE: i32 = 30000000;
    let size = *total_tree.get("/").unwrap();

    *total_tree
        .values()
        .filter(|v| UPDATE + size - **v <= DISK_SIZE)
        .min()
        .unwrap()
}

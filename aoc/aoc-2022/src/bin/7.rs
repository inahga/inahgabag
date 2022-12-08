use std::collections::BinaryHeap;
use std::error::Error;
use std::io::stdin;

const THRESHOLD: usize = 100000;
const DISK_SPACE_AVAILABLE: usize = 70000000;
const DISK_SPACE_NEEDED: usize = 30000000;

#[derive(Debug, Default)]
struct Dir {
    size: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut part1: usize = 0;

    let mut stack = vec![];
    let mut dir_sizes = BinaryHeap::new();
    let mut cwd = Box::new(Dir::default());

    for line in stdin().lines() {
        let l = line?;

        if l == "$ cd .." {
            let s = cwd.size;
            if s <= THRESHOLD {
                part1 += s;
            }
            cwd = stack.pop().expect("stack underflow");
            cwd.size += s;
            dir_sizes.push(s);
        } else if l != "$ cd /" && l.starts_with("$ cd") {
            stack.push(cwd);
            cwd = Box::new(Dir::default());
        } else if l.chars().nth(0).unwrap().is_digit(10) {
            cwd.size += l.split_whitespace().nth(0).unwrap().parse::<usize>()?
        }
    }

    // Pop the stack until we're back at /
    while stack.len() > 0 {
        let s = cwd.size;
        if s <= THRESHOLD {
            part1 += s;
        }
        cwd = stack.pop().unwrap();
        cwd.size += s;
        dir_sizes.push(s);
    }

    let space_needed = DISK_SPACE_NEEDED - (DISK_SPACE_AVAILABLE - cwd.size);
    assert!(space_needed > 0);

    println!("part 1: {}", part1);
    println!(
        "part 2: {}",
        dir_sizes
            .into_sorted_vec()
            .iter()
            .find(|s| **s >= space_needed)
            .unwrap()
    );
    Ok(())
}

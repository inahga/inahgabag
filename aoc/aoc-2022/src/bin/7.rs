use std::error::Error;
use std::io::stdin;

const THRESHOLD: usize = 100000;

#[derive(Debug, Default)]
struct Dir {
    size: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut part1: usize = 0;
    let mut stack = vec![];
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
        } else if l != "$ cd /" && l.starts_with("$ cd") {
            stack.push(cwd);
            cwd = Box::new(Dir::default());
        } else if l.chars().nth(0).unwrap().is_digit(10) {
            cwd.size += l
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
    }

    println!("part 1: {}", part1);
    Ok(())
}

use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;

fn parse_stacks() -> Result<Vec<VecDeque<char>>, Box<dyn Error>> {
    let mut stacks: Vec<VecDeque<char>> = vec![];
    let mut line = String::new();

    'outer: while stdin().read_line(&mut line)? != 0 {
        for (i, c) in line.chars().enumerate() {
            // The start of instructions are denoted by a series of numbers.
            if c.is_numeric() {
                line.clear();
                break 'outer;
            }

            if c.is_alphabetic() {
                let stack_num = i / 4;

                // Ensure the stacks size is long enough before inserting.
                if stacks.len() < stack_num + 1 {
                    stacks.resize(stack_num + 1, VecDeque::new())
                }

                stacks[stack_num].push_front(c);
            }
        }
        line.clear();
    }
    Ok(stacks)
}

fn print_stack(prefix: &str, stacks: Vec<VecDeque<char>>) {
    print!("{}", prefix);
    stacks.iter().for_each(|s| print!("{}", s.back().unwrap()));
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    // Stack is populated from the end to the beginning, so using VecDeque prevents
    // having to reorder the Vec to have it behave as a proper stack.
    let mut stacks = parse_stacks()?;
    let mut fancy_stacks = stacks.clone();

    // By this point, the stdin iterator has advanced to the instruction stage.
    let mut line = String::new();
    while stdin().read_line(&mut line)? != 0 {
        if line.trim().is_empty() {
            continue;
        }

        // Naive parsing of the instruction, just rip out the numbers.
        let mut instruction =
            line.split_ascii_whitespace()
                .filter_map(|s| match s.parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                });
        let (num, from, to) = (
            instruction.next().unwrap(),
            instruction.next().unwrap(),
            instruction.next().unwrap(),
        );

        (0..num).for_each(|_| {
            let insert = stacks[from - 1].pop_back().expect("stack underflow");
            stacks[to - 1].push_back(insert);
        });

        line.clear();
    }

    print_stack("part 1:", stacks);
    print_stack("part 2:", fancy_stacks);
    Ok(())
}

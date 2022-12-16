use std::collections::{BinaryHeap, VecDeque};
use std::error::Error;
use std::io::stdin;

#[derive(Debug, Default, Clone)]
enum Op {
    #[default]
    Multiply,
    Add,
    Square,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    op_factor: i64,
    test_divisor: i64,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

fn parse() -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut line = String::new();
    let mut monkeys = vec![];
    let mut monkey = Monkey::default();

    // World's worst parser, big ooof.
    while stdin().read_line(&mut line)? != 0 {
        // Trim trailing newline.
        line.pop();

        // Treat a blank line as the end of a monkey declaration.
        if line.trim().is_empty() {
            monkeys.push(monkey);
            monkey = Monkey::default();
        } else if line.starts_with("Monkey") {
            // Ignore given indexes, input is already sorted.
        } else {
            let mut split = line.split(":");

            let property = split.next().unwrap().trim();
            let value = split.next().unwrap().trim();
            match property {
                "Starting items" => {
                    monkey.items = value
                        .split(",")
                        .map(|s| s.trim().parse().unwrap())
                        .collect()
                }
                "Operation" => {
                    let mut vals = value.split_whitespace().rev();
                    let factor = vals.next().unwrap();
                    if factor == "old" {
                        monkey.op = Op::Square;
                    } else {
                        monkey.op_factor = factor.parse().unwrap();
                        match vals.next().unwrap() {
                            "*" => monkey.op = Op::Multiply,
                            "+" => monkey.op = Op::Add,
                            _ => panic!("unknown op"),
                        }
                    }
                }
                "Test" => {
                    monkey.test_divisor = value.split_whitespace().last().unwrap().parse().unwrap();
                }
                "If true" => {
                    monkey.if_true = value.split_whitespace().last().unwrap().parse().unwrap();
                }
                "If false" => {
                    monkey.if_false = value.split_whitespace().last().unwrap().parse().unwrap();
                }
                p @ _ => panic!("unknown property {}", p),
            };
        }

        line.clear();
    }
    monkeys.push(monkey);
    Ok(monkeys)
}

fn play(monkeys: &mut Vec<Monkey>, rounds: usize, divide: bool) {
    let divisor = if !divide {
        monkeys
            .iter()
            .map(|m| m.test_divisor)
            .reduce(|a, b| a * b)
            .unwrap()
    } else {
        0
    };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                match monkeys[i].op {
                    Op::Multiply => item *= monkeys[i].op_factor,
                    Op::Add => item += monkeys[i].op_factor,
                    Op::Square => item *= item,
                }
                if divide {
                    item /= 3;
                } else {
                    // Need to reduce the value, while still allowing the next test
                    // to have the same result. We only care about the divisibility,
                    // so whether it's divisible by e.g. 19 or 13 or 17. As it so happens,
                    // if we take the modulo of all values multiplied, this constraint
                    // is satisfied.
                    item %= divisor;
                }

                let m = if item % monkeys[i].test_divisor == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[m].items.push_back(item);
                monkeys[i].inspected += 1;
            }
        }
    }
    println!(
        "{:?}",
        monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>()
    );
}

fn monkey_business(monkeys: &Vec<Monkey>) -> usize {
    monkeys
        .iter()
        .map(|m| m.inspected)
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .iter()
        .rev()
        .take(2)
        .product()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut part1 = parse()?;
    let mut part2 = part1.clone();
    play(&mut part1, 20, true);
    play(&mut part2, 10000, false);

    println!("part 1: {}", monkey_business(&part1));
    println!("part 2: {}", monkey_business(&part2));

    Ok(())
}

use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;

fn signum(i: i64) -> i64 {
    match i {
        i if i > 0 => 1,
        0 => 0,
        _ => -1,
    }
}

fn simulate(instructions: &Vec<(String, usize)>, n: usize) -> usize {
    let mut segments = vec![(0i64, 0i64); n];
    let mut tpositions: HashSet<(i64, i64)> = HashSet::new();

    for instruction in instructions {
        let direction = &instruction.0;
        let steps = instruction.1;

        for _ in 0..steps {
            let head = &mut segments[0];
            match direction.as_str() {
                "L" => *head = (head.0 - 1, head.1),
                "R" => *head = (head.0 + 1, head.1),
                "U" => *head = (head.0, head.1 + 1),
                "D" => *head = (head.0, head.1 - 1),
                _ => panic!("unknown direction"),
            };

            // Propagate changes to the remaining segments.
            for s in 0..segments.len() - 1 {
                let (left, right) = segments.split_at_mut(s + 1);
                let head = &mut left[s];
                let tail = &mut right[0];

                // Check if the tail segment is too far away.
                if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    // Move once in the general direction of the head.
                    *tail = (
                        tail.0 + signum(head.0 - tail.0),
                        tail.1 + signum(head.1 - tail.1),
                    );
                }
            }
            tpositions.insert(*segments.last().unwrap());
        }
    }
    tpositions.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin()
        .lines()
        .map(|line| {
            let u = line.unwrap();
            let mut l = u.split_ascii_whitespace();
            (
                String::from(l.next().unwrap()),
                l.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    println!("part 1: {}", simulate(&lines, 2));
    println!("part 2: {}", simulate(&lines, 10));
    Ok(())
}

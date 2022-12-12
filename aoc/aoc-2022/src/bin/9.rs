use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn new(c: &str) -> Option<Direction> {
        match c {
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            _ => None,
        }
    }
}

type Pair = (i64, i64);

fn far(a: Pair, b: Pair) -> bool {
    (a.0 - b.0).abs() > 1 || (a.1 - b.1).abs() > 1
}

fn mv(a: Pair, d: Direction) -> Pair {
    match d {
        Direction::Left => (a.0 - 1, a.1),
        Direction::Right => (a.0 + 1, a.1),
        Direction::Up => (a.0, a.1 + 1),
        Direction::Down => (a.0, a.1 - 1),
    }
}

fn mv_towards(head: &Pair, tail: &Pair) -> Pair {
    (
        if (head.0 - tail.0) == 0 {
            tail.0
        } else {
            tail.0 + (head.0 - tail.0) / (head.0 - tail.0).abs()
        },
        if (head.1 - tail.1) == 0 {
            tail.1
        } else {
            tail.1 + (head.1 - tail.1) / (head.1 - tail.1).abs()
        },
    )
}

fn simulate(instructions: &Vec<(Direction, usize)>, n: usize) -> usize {
    let mut segments = vec![(0i64, 0i64) as Pair; n];
    let mut tpositions: HashSet<Pair> = HashSet::new();

    for instruction in instructions {
        let direction = instruction.0;
        let steps = instruction.1;

        for _ in 0..steps {
            segments[0] = mv(segments[0], direction);

            // Iterate pairs of segments.
            for s in 0..segments.len() - 1 {
                let (left, right) = segments.split_at_mut(s + 1);
                let head = &mut left[s];
                let tail = &mut right[0];

                if far(*head, *tail) {
                    *tail = mv_towards(head, tail);
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
                Direction::new(l.next().unwrap()).unwrap(),
                l.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    println!("part 1: {}", simulate(&lines, 2));
    println!("part 2: {}", simulate(&lines, 10));
    Ok(())
}

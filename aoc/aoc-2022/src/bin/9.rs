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

fn main() -> Result<(), Box<dyn Error>> {
    let mut head: Pair = (0, 0);
    let mut tail: Pair = (0, 0);
    let mut tpositions: HashSet<Pair> = HashSet::new();

    let mut line = String::new();
    while stdin().read_line(&mut line)? != 0 {
        let mut l = line.split_ascii_whitespace();
        let (direction, steps) = (
            Direction::new(l.next().unwrap()).unwrap(),
            l.next().unwrap().parse::<usize>().unwrap(),
        );

        for _ in 0..steps {
            head = mv(head, direction);
            if far(head, tail) {
                // Always move in the same direction.
                tail = mv(tail, direction);

                // If we aren't in the same row and same column, we have to move
                // diagonally.
                if head.0 != tail.0 && head.1 != tail.1 {
                    // The move earlier completed the first part of diagnonal movement,
                    // now move perpendicular to that.
                    match direction {
                        Direction::Left | Direction::Right => tail = (tail.0, head.1),
                        Direction::Up | Direction::Down => tail = (head.0, tail.1),
                    }
                }
            }
            tpositions.insert(tail);
        }

        line.clear();
    }

    println!("part 1: {}", tpositions.len());
    Ok(())
}

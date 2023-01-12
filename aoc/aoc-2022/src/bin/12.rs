use std::cell::Cell;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;

type Point = (usize, usize);

#[derive(Debug, Copy, Clone)]
struct Plot {
    value: char,
    last: Option<Point>,
}

type Grid = Vec<Vec<Cell<Plot>>>;

fn parse() -> Result<(Grid, Point), Box<dyn Error>> {
    let mut ret = vec![];
    let mut start = (0, 0);
    let mut i = 0;

    let mut line = String::new();
    while stdin().read_line(&mut line)? != 0 {
        line.pop();

        let mut vec_line = vec![];
        for (j, c) in line.chars().enumerate() {
            vec_line.push(Cell::new(Plot {
                value: c,
                last: None,
            }));
            if c == 'S' {
                start = (i, j);
            }
        }

        line.clear();
        ret.push(vec_line);
        i += 1;
    }
    Ok((ret, start))
}

fn candidates(p: Point, dim: usize) -> impl Iterator<Item = Point> {
    let mut ret = vec![];
    ret.push((p.0 + 1, p.1));
    ret.push((p.0, p.1 + 1));
    if let Some(l) = p.0.checked_sub(1) {
        ret.push((l, p.1));
    }
    if let Some(u) = p.1.checked_sub(1) {
        ret.push((p.0, u));
    }
    ret.into_iter()
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut grid, start) = parse()?;
    let mut found = None;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let curr_pt = queue.pop_front().unwrap();
        let curr_plot = grid[curr_pt.0][curr_pt.1].get();

        for candidate in candidates(curr_pt) {
            let candidate_plot = grid[candidate.0][candidate.1].get_mut();
            candidate_plot.last = Some(curr_pt);

            if candidate_plot.value == 'E' {
                found = Some(candidate_plot.clone());
                break;
            }

            let diff = if curr_plot.value == 'S' {
                candidate_plot.value as i16 - ('a' as i16 - 1)
            } else {
                candidate_plot.value as i16 - curr_plot.value as i16
            };
            if diff == 1 || diff == 0 {
                queue.push_back(candidate);
            }
        }
    }

    // start at E, backtrack last until we find s
    println!("{:?}", grid);
    println!("{:?}", found);

    Ok(())
}

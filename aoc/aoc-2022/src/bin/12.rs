use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::io::stdin;

type Point = (usize, usize);

#[derive(Debug, Copy, Clone)]
struct Plot {
    value: char,
    last: Option<Point>,
    visited: bool,
}

type Grid = Vec<Vec<RefCell<Plot>>>;

fn parse() -> Result<(Grid, Point), Box<dyn Error>> {
    let mut ret = vec![];
    let mut start = (0, 0);
    let mut i = 0;

    let mut line = String::new();
    while stdin().read_line(&mut line)? != 0 {
        line.pop();

        let mut vec_line = vec![];
        for (j, c) in line.chars().enumerate() {
            vec_line.push(RefCell::new(Plot {
                value: c,
                last: None,
                visited: false,
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

fn candidates(p: Point) -> impl Iterator<Item = Point> {
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
    let (grid, start) = parse()?;
    let mut found = None;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    {
        let mut start_plot = grid[start.0][start.1].borrow_mut();
        start_plot.visited = true;
    }
    while !queue.is_empty() {
        let curr_pt = queue.pop_front().unwrap();
        let curr_plot = grid[curr_pt.0][curr_pt.1].borrow();

        for candidate in candidates(curr_pt) {
            if candidate.0 >= grid.len() || candidate.1 >= grid[0].len() {
                continue;
            }

            let mut candidate_plot = grid[candidate.0][candidate.1].borrow_mut();

            if candidate_plot.value == 'E' && (curr_plot.value == 'z' || curr_plot.value == 'y') {
                candidate_plot.last = Some(curr_pt);
                candidate_plot.visited = true;
                found = Some(candidate_plot.clone());
                break;
            }

            let diff = if curr_plot.value == 'S' {
                candidate_plot.value as i16 - ('a' as i16)
            } else {
                candidate_plot.value as i16 - curr_plot.value as i16
            };
            if !candidate_plot.visited && diff <= 1 {
                candidate_plot.last = Some(curr_pt);
                candidate_plot.visited = true;
                queue.push_back(candidate);
            }
        }
    }

    let mut part1 = 0;
    let mut iter = found.unwrap();
    while let Some(i) = iter.last {
        println!("{:?}", iter);
        part1 += 1;
        iter = grid[i.0][i.1].borrow().clone();
    }

    println!("{:?}", part1);

    Ok(())
}

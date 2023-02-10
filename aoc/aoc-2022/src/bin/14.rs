use std::cell::{Cell, RefCell};
use std::io::{self, stdin};
use std::ops::{Range, RangeInclusive};
use std::{error::Error, fmt};

const HEIGHT: usize = 150;
const WIDTH: usize = 200;
const OFFSET: usize = 400;

type Point = (usize, usize);

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Rest,
    Rock,
    Sand,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "+"),
            Self::Rest => write!(f, "o"),
        }
    }
}

struct Grid(Vec<Vec<Cell<Tile>>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|row| {
            row.iter()
                .try_for_each(|tile| write!(f, "{}", tile.get()))?;
            write!(f, "\n")
        })
    }
}

fn split_pair(s: &str) -> Option<(usize, usize)> {
    let mut i = s.split(",");
    Some((i.next()?.parse().ok()?, i.next()?.parse().ok()?))
}

fn absolute_range<T: Ord + Copy>(a: T, b: T) -> RangeInclusive<T> {
    a.min(b)..=a.max(b)
}

fn parse() -> Result<Grid, io::Error> {
    let grid = Grid(
        (0..HEIGHT)
            .map(|_| (0..WIDTH).map(|_| Cell::new(Tile::Air)).collect())
            .collect(),
    );

    let mut line = String::new();
    while stdin().read_line(&mut line)? != 0 {
        line.pop();
        line.split(" -> ")
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|elem| {
                let (lx, ly) = split_pair(elem[0]).unwrap();
                let (rx, ry) = split_pair(elem[1]).unwrap();
                absolute_range(ly, ry).for_each(|y| grid.0[y][lx - OFFSET].set(Tile::Rock));
                absolute_range(lx, rx).for_each(|x| grid.0[ly][x - OFFSET].set(Tile::Rock));
            });
        line.clear();
    }

    Ok(grid)
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = parse()?;
    println!("{}", grid);
    Ok(())
}

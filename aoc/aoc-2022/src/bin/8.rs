use std::error::Error;
use std::io::stdin;
use std::ops::ControlFlow;

fn main() -> Result<(), Box<dyn Error>> {
    let input = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut max = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let h = input[row][col];
            if input[row + 1..].iter().all(|e| e[col] < h)
                || input[..row].iter().all(|e| e[col] < h)
                || input[row][col + 1..].iter().all(|e| *e < h)
                || input[row][..col].iter().all(|e| *e < h)
            {
                part1 += 1;
            }

            let f = |s: i32, e: u32| {
                if e >= h {
                    ControlFlow::Break(s + 1)
                } else {
                    ControlFlow::Continue(s + 1)
                }
            };
            let u = |c: ControlFlow<_, _>| match c {
                ControlFlow::Break(v) => v,
                ControlFlow::Continue(v) => v,
            };
            let up = u(input[..row].iter().rev().try_fold(0, |s, e| f(s, e[col])));
            let down = u(input[row + 1..].iter().try_fold(0, |s, e| f(s, e[col])));
            let right = u(input[row][col + 1..].iter().try_fold(0, |s, e| f(s, *e)));
            let left = u(input[row][..col].iter().rev().try_fold(0, |s, e| f(s, *e)));

            let v = up * down * left * right;
            if v > max {
                max = v;
            }
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", max);
    Ok(())
}

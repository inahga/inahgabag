use std::error::Error;
use std::io::stdin;

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
        }
    }

    println!("part 1: {}", part1);
    Ok(())
}

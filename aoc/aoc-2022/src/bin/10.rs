use std::error::Error;
use std::io::stdin;

#[derive(Debug)]
enum State {
    ADDING,
    READY,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::READY;
    let mut reg_x = 1;
    let mut operand = 0;
    let mut cycle = 1i32;
    let mut part1 = 0;

    let mut sprite = [false; 40 * 6];

    let mut line = String::new();
    loop {
        let sprite_idx = (cycle - 1) as usize;
        if (reg_x - 1..=reg_x + 1).contains(&((cycle - 1) % 40)) {
            sprite[sprite_idx] = true;
        }

        match state {
            State::ADDING => {
                reg_x += operand;
                state = State::READY;
            }
            State::READY => {
                let num_read = stdin().read_line(&mut line)?;
                if num_read == 0 {
                    break;
                }

                let mut l = line.split_whitespace();
                match l.next().unwrap() {
                    "addx" => {
                        operand = l.next().unwrap().parse().unwrap();
                        state = State::ADDING;
                    }
                    "noop" => {}
                    _ => panic!("unknown instruction"),
                };
                line.clear();
            }
        }

        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            part1 += cycle * reg_x;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2:");

    sprite.chunks(40).for_each(|l| {
        l.iter()
            .for_each(|c| print!("{}", if *c { "#" } else { "." }));
        println!();
    });
    Ok(())
}

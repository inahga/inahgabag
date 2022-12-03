use std::error::Error;
use std::io::stdin;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn play(&self, other: &RPS) -> i32 {
        let self_val = *self as i32;
        if self == other {
            self_val + 3
        } else if (*other == RPS::Rock && *self == RPS::Paper)
            || (*other == RPS::Paper && *self == RPS::Scissors)
            || (*other == RPS::Scissors && *self == RPS::Rock)
        {
            self_val + 6
        } else {
            self_val
        }
    }

    // result returns what needs to be played against self to achieve the desired
    // result.
    fn result(&self, result: &RPSResult) -> RPS {
        match result {
            RPSResult::Draw => *self,
            RPSResult::Win => match self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            RPSResult::Lose => match self {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
        }
    }

    fn new(c: &str) -> Option<RPS> {
        match c {
            "A" | "X" => Some(RPS::Rock),
            "B" | "Y" => Some(RPS::Paper),
            "C" | "Z" => Some(RPS::Scissors),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum RPSResult {
    Win,
    Lose,
    Draw,
}

impl RPSResult {
    fn new(c: &str) -> Option<RPSResult> {
        match c {
            "X" => Some(RPSResult::Lose),
            "Y" => Some(RPSResult::Draw),
            "Z" => Some(RPSResult::Win),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in stdin().lines() {
        let l = line?;
        let mut split = l.split_ascii_whitespace();
        let (theirs, ours) = (split.next().unwrap(), split.next().unwrap());

        let (rpstheirs, rpsours) = (RPS::new(theirs).unwrap(), RPS::new(ours).unwrap());
        part1 += rpsours.play(&rpstheirs);

        let to_play = // what our hand is going to be
	        &rpstheirs.result(&RPSResult::new(ours).unwrap());

        part2 += to_play.play(&rpstheirs);
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
    Ok(())
}

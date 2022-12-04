use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in stdin().lines() {
        let l = line?;

        // We'll just assume the input is well formatted.
        let groups = l
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let (lfirst, rfirst, lsecond, rsecond) = (groups[0], groups[1], groups[2], groups[3]);

        // Pairs that are subsets.
        if (lfirst <= lsecond && rfirst >= rsecond) || (lfirst >= lsecond && rfirst <= rsecond) {
            part1 += 1;
        }

        // Pairs that overlap at all.
        if rfirst >= lsecond && lfirst <= rsecond {
            part2 += 1;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

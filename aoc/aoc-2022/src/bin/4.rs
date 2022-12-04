use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    let mut part1 = 0;
    let mut part2 = 0;

    while stdin().read_line(&mut line)? != 0 {
        // Remove trailing newline.
        line.pop();

        // We'll just assume the input is well formatted.
        let mut groups = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<u64>().unwrap());

        let (lfirst, rfirst, lsecond, rsecond) = (
            groups.next().unwrap(),
            groups.next().unwrap(),
            groups.next().unwrap(),
            groups.next().unwrap(),
        );

        // Pairs that are subsets.
        if (lfirst <= lsecond && rfirst >= rsecond) || (lfirst >= lsecond && rfirst <= rsecond) {
            part1 += 1;
        }

        // Pairs that overlap at all.
        if rfirst >= lsecond && lfirst <= rsecond {
            part2 += 1;
        }

        line.clear();
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}

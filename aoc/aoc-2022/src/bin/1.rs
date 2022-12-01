use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin().lines();

    let mut sums = vec![];
    let mut sum: u64 = 0;

    for line in lines {
        let l = line?;
        if l.is_empty() && sum != 0 {
            sums.push(sum);
            sum = 0;
            continue;
        }
        sum += l.parse::<u64>()?;
    }
    if sum > 0 {
        sums.push(sum);
    }

    // part 1
    println!("{}", sums.iter().max().unwrap());

    // part 2
    sums.sort_unstable_by(|a, b| b.cmp(&a));
    println!("{}", &sums[..3].iter().sum::<u64>());

    Ok(())
}

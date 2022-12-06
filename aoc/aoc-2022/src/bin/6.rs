use std::error::Error;
use std::io::stdin;

fn first_unique_substring(length: usize, line: &String) -> Option<usize> {
    Some(
        line.as_bytes()
            .windows(length)
            .enumerate()
            .find(|(_, w)| {
                let mut n: u64 = 0;
                for c in w.iter() {
                    let i = 1 << (*c as u64 - 65);
                    if n & i > 0 {
                        return false;
                    }
                    n |= i;
                }
                true
            })?
            .0
            + length,
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let line = stdin().lines().next().unwrap()?;
    println!("part 1: {}", first_unique_substring(4, &line).unwrap());
    println!("part 2: {}", first_unique_substring(14, &line).unwrap());
    Ok(())
}

#[test]
fn test_part1() {
    assert!(
        first_unique_substring(4, &String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).unwrap() == 7
    );
    assert!(first_unique_substring(4, &String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")).unwrap() == 5);
    assert!(first_unique_substring(4, &String::from("nppdvjthqldpwncqszvftbrmjlhg")).unwrap() == 6);
    assert!(
        first_unique_substring(4, &String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).unwrap()
            == 10
    );
    assert!(
        first_unique_substring(4, &String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).unwrap() == 11
    );
    assert!(
        first_unique_substring(14, &String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).unwrap() == 19
    );
    assert!(
        first_unique_substring(14, &String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")).unwrap() == 23
    );
    assert!(
        first_unique_substring(14, &String::from("nppdvjthqldpwncqszvftbrmjlhg")).unwrap() == 23
    );
    assert!(
        first_unique_substring(14, &String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).unwrap()
            == 29
    );
    assert!(
        first_unique_substring(14, &String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).unwrap()
            == 26
    );
}

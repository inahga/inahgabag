use std::collections::HashSet;
use std::error::Error;
use std::io::stdin;

fn priority(c: char) -> Option<u64> {
    match c {
        x @ 'a'..='z' => Some(x as u64 - 96),
        x @ 'A'..='Z' => Some(x as u64 - 65 + 27),
        _ => None,
    }
}

fn intersection(s: Vec<String>) -> Option<HashSet<char>> {
    if s.is_empty() {
        return None;
    }
    let mut ret = s[0].chars().collect::<HashSet<_>>();
    for str in &s[1..] {
        ret = str.chars().filter(|c| ret.contains(c)).collect();
    }
    Some(ret)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut part1, mut part2) = (0, 0);
    let mut group: Vec<String> = Vec::new();

    fn intersection_priority(s: Vec<String>) -> u64 {
        intersection(s)
            .unwrap()
            .iter()
            .map(|c| priority(*c).unwrap())
            .sum()
    }
    for (i, line) in stdin().lines().enumerate() {
        let l = line?;

        let split = l.split_at(l.len() / 2);
        part1 += intersection_priority(vec![String::from(split.0), String::from(split.1)]);

        if i > 0 && i % 3 == 0 {
            part2 += intersection_priority(group);
            group = Vec::new();
        }
        group.push(l);
    }
    part2 += intersection_priority(group);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
    Ok(())
}

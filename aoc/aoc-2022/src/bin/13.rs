use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::error::Error;
use std::io::stdin;

#[derive(Debug)]
enum Node {
    Num(i32),
    Node(Vec<Box<Node>>),
}

impl Node {
    fn from(s: &str) -> Self {
        let mut stack = vec![];
        let mut num = String::new();

        for c in s.chars() {
            match c {
                '[' => stack.push(Box::new(Self::Node(vec![]))),
                ',' | ']' => {
                    if num.len() > 0 {
                        if let Self::Node(node) = stack.last_mut().unwrap().borrow_mut() {
                            node.push(Box::new(Node::Num(
                                num.parse().expect("unable to parse number"),
                            )));
                            num.clear();
                        }
                    }

                    if c == ']' && stack.len() > 1 {
                        let last = stack.pop().unwrap();
                        if let Self::Node(node) = stack.last_mut().unwrap().borrow_mut() {
                            node.push(last);
                        }
                    }
                }
                '0'..='9' => num.push(c),
                _ => panic!("unknown character {}", c),
            }
        }

        return *stack.pop().unwrap();
    }
}

fn compare(left: &Node, right: &Node) -> Ordering {
    match (left, right) {
        (Node::Num(l), Node::Num(r)) => return l.cmp(r),
        (Node::Node(l), Node::Node(r)) => {
            for (i, lnode) in l.iter().enumerate() {
                if i >= r.len() {
                    return Ordering::Greater;
                }
                match compare(lnode, &r[i]) {
                    Ordering::Equal => continue,
                    res => return res,
                }
            }
            if l.len() < r.len() {
                return Ordering::Less;
            }
            return Ordering::Equal;
        }
        (Node::Num(l), Node::Node(_)) => {
            return compare(&Node::Node(vec![Box::new(Node::Num(*l))]), right)
        }
        (Node::Node(_), Node::Num(r)) => {
            return compare(left, &Node::Node(vec![Box::new(Node::Num(*r))]))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines: Vec<String> = stdin()
        .lines()
        .filter_map(|line| {
            let l = line.unwrap();
            if l.is_empty() {
                None
            } else {
                Some(l)
            }
        })
        .collect();

    let part1 = lines.chunks(2).enumerate().fold(0, |last, pair| {
        match compare(&Node::from(&pair.1[0]), &Node::from(&pair.1[1])) {
            Ordering::Less => last + pair.0 + 1,
            _ => last,
        }
    });
    println!("part 1: {}", part1);

    lines.push(String::from("[[2]]"));
    lines.push(String::from("[[6]]"));
    lines.sort_by(|a, b| compare(&Node::from(a), &Node::from(b)));
    let part2 = [2, 6].iter().fold(1, |last, target| {
        last * (lines
            .binary_search_by(|line| {
                compare(
                    &Node::from(line),
                    &Node::Node(vec![Box::new(Node::Node(vec![Box::new(Node::Num(
                        *target,
                    ))]))]),
                )
            })
            .unwrap()
            + 1)
    });
    println!("part 2: {}", part2);

    Ok(())
}

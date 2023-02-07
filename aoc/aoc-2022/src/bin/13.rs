use std::borrow::BorrowMut;
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

enum Valid {
    False,
    True,
    Inconclusive,
}

fn compare(left: &Node, right: &Node) -> Valid {
    if let (Node::Num(l), Node::Num(r)) = (left, right) {
        if l < r {
            return Valid::True;
        } else if l > r {
            return Valid::False;
        } else {
            return Valid::Inconclusive;
        }
    } else if let (Node::Node(l), Node::Node(r)) = (left, right) {
        for (i, lnode) in l.iter().enumerate() {
            if i >= r.len() {
                return Valid::False;
            }
            match compare(lnode, &r[i]) {
                Valid::Inconclusive => continue,
                res => return res,
            }
        }
        if l.len() < r.len() {
            return Valid::True;
        } else {
            return Valid::Inconclusive;
        }
    } else if let (Node::Num(l), Node::Node(_)) = (left, right) {
        return compare(&Node::Node(vec![Box::new(Node::Num(*l))]), right);
    } else if let (Node::Node(_), Node::Num(r)) = (left, right) {
        return compare(left, &Node::Node(vec![Box::new(Node::Num(*r))]));
    }
    unreachable!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let part1 = stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(Node::from(&line))
            }
        })
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .fold(0, |last, pair| {
            if let Valid::True = compare(&pair.1[0], &pair.1[1]) {
                last + pair.0 + 1
            } else {
                last
            }
        });
    println!("part 1: {}", part1);

    Ok(())
}

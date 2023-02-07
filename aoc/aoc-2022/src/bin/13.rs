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

fn main() -> Result<(), Box<dyn Error>> {
    let lines = stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(Node::from(&line))
            }
        })
        .inspect(|node| println!("{:?}", node))
        .collect::<Vec<_>>()
        .chunks(2)
        .fold(0, |last, pair| {
            let left = &pair[0];
            let right = &pair[1];

            last
        });

    Ok(())
}

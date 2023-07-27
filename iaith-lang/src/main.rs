use core::panic;
use std::io::{stdin, stdout, Write};

enum Node {
    Addition,
    Subtraction,
    Integer(i64),
    List(Vec<Node>),
}

fn read() -> Result<Node, std::io::Error> {
    let mut stdout = stdout();
    let stdin = stdin();
    let mut input = String::new();

    print!("\niaith-lang> ");
    stdout.flush()?;
    stdin.read_line(&mut input)?;

    let mut stack: Vec<Node> = Vec::new();
    stack.push(Node::List(Vec::new()));

    for c in input.chars() {
        match c {
            '+' => match stack.last_mut() {
                Some(Node::List(list)) => list.push(Node::Addition),
                _ => panic!("Expected list"),
            },
            '-' => match stack.last_mut() {
                Some(Node::List(list)) => list.push(Node::Subtraction),
                _ => panic!("Expected list"),
            },
            '(' => {
                stack.push(Node::List(Vec::new()));
            }
            ')' => {
                let node = stack.pop().unwrap();
                match stack.last_mut() {
                    Some(Node::List(list)) => list.push(node),
                    _ => panic!("Expected list"),
                }
            }
            ' ' | '\n' | '\t' => {}
            '0'..='9' => match stack.last_mut() {
                Some(Node::List(list)) => list.push(Node::Integer(c.to_digit(10).unwrap() as i64)),
                _ => panic!("Expected list"),
            },
            _ => panic!("Unexpected character"),
        }
    }

    if stack.len() != 1 {
        panic!("Unclosed parenthesis");
    }

    match stack.pop().unwrap() {
        Node::List(list) => Ok(Node::List(list)),
        _ => panic!("Expected list"),
    }
}

fn eval(nodes: Node) -> Node {
    nodes
}

fn print_node(node: Node, is_root: bool) -> String {
    match node {
        Node::Addition => "+".to_string(),
        Node::Subtraction => "-".to_string(),
        Node::Integer(n) => n.to_string(),
        Node::List(list) => {
            let mut output = String::new();

            if !is_root {
                output.push_str(&format!("("));
            }

            for node in list {
                output.push_str(&print_node(node, false));
                output.push_str(" ");
            }
            output.pop();

            if !is_root {
                output.push_str(&format!(")"));
            }

            output
        }
    }
}

fn print(nodes: Node) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    print!("{}", print_node(nodes, true));
    stdout.flush()?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    loop {
        let line = read()?;
        let evaluated = eval(line);
        print(evaluated)?;
    }
}

use core::panic;
use std::io::{stdin, stdout, Write};

enum Node {
    Addition,
    Subtraction,
    Integer(i64),
    List(Vec<Node>),
}

fn read() -> Result<Vec<Node>, std::io::Error> {
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
            ' ' => (),
            '\n' => (),
            _ => panic!("Unexpected character"),
        }
    }

    if stack.len() != 1 {
        panic!("Unclosed parenthesis");
    }

    match stack.pop().unwrap() {
        Node::List(list) => Ok(list),
        _ => panic!("Expected list"),
    }
}

fn eval(nodes: Vec<Node>) -> String {
    let mut result = String::new();
    result
}

fn print(line: String) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    print!("{}", line);
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

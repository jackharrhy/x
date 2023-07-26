use std::io::{stdin, stdout, Write};

fn read() -> Result<String, std::io::Error> {
    let mut stdout = stdout();
    let stdin = stdin();
    let mut input = String::new();

    print!("iaith-lang> ");
    stdout.flush()?;
    stdin.read_line(&mut input)?;

    Ok(input)
}

fn eval(line: String) -> String {
    line
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

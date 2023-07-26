use nom::{
    bytes::complete::{tag, take_till},
    character::is_newline,
    IResult,
};

fn iaith_file(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("🏴󠁧󠁢󠁷󠁬󠁳󠁿")(input)?;
    let (input, _) = tag("󠁷󠁬󠁳\n")(input)?;

    // let (input, line) = take_till(is_newline)(input)?;

    // print line
    // println!("Line: {}", line);

    Ok((input, ()))
}

fn main() {
    let contents =
        std::fs::read_to_string("example.iaith").expect("Something went wrong reading the file");

    let res = iaith_file(&contents).expect("Failed to parse file");

    println!("Parsed file: {:?}", res);

    // print the contents of the file
    println!("With text:\n{}", contents);
}

use anyhow::Result;

use std::{fs::File, io::Read};

use nom::{bytes::complete::tag, IResult};

const TPL_FILE_IDENTIFIER: [u8; 4] = [0x00, 0x20, 0xAF, 0x30];

fn parse_tpl(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag(TPL_FILE_IDENTIFIER)(input)?;
    Ok((input, ()))
}

fn read_file_bytes(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut file = File::open("savebanner.tpl")?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("read bytes from file");
    parse_tpl(&bytes)?;

    Ok(())
}

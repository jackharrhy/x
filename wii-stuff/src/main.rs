use anyhow::Result;

use std::{fs::File, io::Read};

use nom::{
    bytes::complete::{tag, take},
    IResult,
};

const TPL_FILE_IDENTIFIER: [u8; 4] = [0x00, 0x20, 0xAF, 0x30];

fn parse_tpl(input: &[u8]) -> IResult<&[u8], ()> {
    let mut pos = 0;
    let (input, _) = tag(TPL_FILE_IDENTIFIER)(input)?;
    pos += 4;

    let (input, image_count) = take(4u8)(input)?;
    pos += 4;
    let image_count = u32::from_be_bytes(image_count.try_into().unwrap());
    println!("image count: {:?}", image_count);

    let (input, image_table_offset) = take(4u8)(input)?;
    pos += 4;
    let image_table_offset = u32::from_be_bytes(image_table_offset.try_into().unwrap());
    println!("image table offset: {:?}", image_table_offset);

    let move_by = image_table_offset - pos;
    println!("move by: {:?}", move_by);

    let (input, _) = take(move_by)(input)?;

    // for i in 0..image_count {}

    Ok((input, ()))
}

fn main() -> Result<()> {
    let mut file = File::open("savebanner.tpl")?;

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect("read bytes from file");

    let bytes = &bytes[..];
    parse_tpl(bytes).expect("failed to parse tpl");

    Ok(())
}

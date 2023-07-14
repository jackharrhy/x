use std::{fs::File, io::Result};

fn main() -> Result<()> {
    println!("Hello, world!");

    // read in a file called savebanner.tpl in the current dir as bytes
    let mut file = File::open("savebanner.tpl")?;

    Ok(())
}

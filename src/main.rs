use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
mod header;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            let mut page_header = [0;12];
            file.read_exact(&mut header)?;
            file.read_exact(&mut page_header)?;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            let page_size = u16::from_be_bytes([header[16], header[17]]);
            println!("database page size: {}", page_size);
            let table_number = header::get_number_of_tables(page_header);
            println!("number of tables: {}", table_number);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}

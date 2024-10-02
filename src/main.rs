mod header;
mod sqlite;

use crate::sqlite::database_header::Header;
use crate::sqlite::utils::read_int;
use anyhow::{bail, Result};
use sqlite::page_header::PageHeader;
use sqlite::record::{ColumnValue, Record};
use std::fs::File;
use std::io::{prelude::*, SeekFrom};

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
            let header = Header::read(&mut file)?;
            let mut page_header = [0; 12];
            file.read_exact(&mut page_header)?;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            println!("database page size: {}", header.page_size);
            let table_number = header::get_number_of_tables(page_header);
            println!("number of tables: {}", table_number);
        }
        ".tables" => {
            let mut file = File::open(&args[1])?;
            let _header = Header::read(&mut file)?;

            let first_page_header = PageHeader::read(&mut file)?;
            let mut cell_pointers = Vec::with_capacity(first_page_header.cells_count.into());

            for _ in 0..first_page_header.cells_count {
                let mut cell_pointer = [0; 2];
                file.read_exact(&mut cell_pointer)?;
                cell_pointers.push(u16::from_be_bytes(cell_pointer));
            }

            for cell_pointer in cell_pointers {
                file.seek(SeekFrom::Start(cell_pointer as u64))?;
                let _payload = read_int(&mut file)?;
                let _row = read_int(&mut file)?;
                let record = Record::read(&mut file)?;

                if let ColumnValue::Text(ref name) = record.values[1] {
                    let name = String::from_utf8_lossy(&name);
                    if name != "sqlite_sequence" {
                        print!("{} ", name);
                    }
                }
            }
            print!("\n");
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}

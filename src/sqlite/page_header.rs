use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Debug)]
pub enum PageType {
    InteriorIndexPage,
    InteriorTablePage,
    LeafIndexPage,
    LeafTablePage,
}

pub struct PageHeader {
    pub page_type: PageType,
    pub first_freeblock: u16,
    pub cells_count: u16,
    pub cell_content_start: u16,
    pub fragmented_free_bytes: u8,
}

impl PageHeader {
    pub fn read(file: &mut File) -> Result<Self> {
        let mut bytes = [0; 8];
        file.read_exact(&mut bytes)?;
        Ok(Self {
            page_type: match bytes[0] {
                0x02 => PageType::InteriorIndexPage,
                0x05 => PageType::InteriorTablePage,
                0x0a => PageType::LeafIndexPage,
                0x0d => PageType::LeafTablePage,
                _ => unreachable!(),
            },
            first_freeblock: u16::from_be_bytes([bytes[1], bytes[2]]),
            cells_count: u16::from_be_bytes([bytes[3], bytes[4]]),
            cell_content_start: u16::from_be_bytes([bytes[5], bytes[6]]),
            fragmented_free_bytes: u8::from_be(bytes[7]),
        })
    }
}

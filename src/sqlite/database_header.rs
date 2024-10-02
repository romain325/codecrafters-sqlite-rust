use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::str;

// copied from Cnidarias solution bc i'm way to lazy sry
pub struct Header {
    pub header_string: String,
    pub page_size: u16,
    pub file_format_write_version: u8,
    pub file_format_read_version: u8,
    pub end_of_page_reserved_space: u8,
    pub maximum_embedded_payload_fraction: u8,
    pub minimum_embedded_payload_fraction: u8,
    pub leaf_payload_fraction: u8,
    pub file_change_counter: u32,
    pub size_in_pages: u32,
    pub page_number_of_the_first_freelist_trunk_page: u32,
    pub total_freelist_pages: u32,
    pub schema_cookie: u32,
    pub schema_format_number: u32,
    pub default_page_cache_size: u32,
    pub largest_root_page: u32,
    pub text_encoding: u32,
    pub user_version: u32,
    pub incremental_vacuum_mode: bool,
    pub application_id: u32,
    pub version_valid_for_number: u32,
    pub version: u32,
}

impl Header {
    pub fn read(file: &mut File) -> Result<Self> {
        let mut bytes = [0; 100];
        file.read_exact(&mut bytes)?;
        Ok(Self {
            header_string: str::from_utf8(&bytes[0..16])?.to_string(),
            page_size: u16::from_be_bytes([bytes[16], bytes[17]]),
            file_format_write_version: u8::from_be(bytes[18]),
            file_format_read_version: u8::from_be(bytes[19]),
            end_of_page_reserved_space: u8::from_be(bytes[20]),
            maximum_embedded_payload_fraction: u8::from_be(bytes[21]),
            minimum_embedded_payload_fraction: u8::from_be(bytes[22]),
            leaf_payload_fraction: u8::from_be(bytes[23]),
            file_change_counter: u32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
            size_in_pages: u32::from_be_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
            page_number_of_the_first_freelist_trunk_page: u32::from_be_bytes([
                                                                             bytes[32], bytes[33], bytes[34], bytes[35],
            ]),
            total_freelist_pages: u32::from_be_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]),
            schema_cookie: u32::from_be_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]),
            schema_format_number: u32::from_be_bytes([bytes[44], bytes[45], bytes[46], bytes[47]]),
            default_page_cache_size: u32::from_be_bytes([
                                                        bytes[48], bytes[49], bytes[50], bytes[51],
            ]),
            largest_root_page: u32::from_be_bytes([bytes[52], bytes[53], bytes[54], bytes[55]]),
            text_encoding: u32::from_be_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]),
            user_version: u32::from_be_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]),
            incremental_vacuum_mode: u32::from_be_bytes([
                                                        bytes[64], bytes[65], bytes[66], bytes[67],
            ]) != 0,
            application_id: u32::from_be_bytes([bytes[68], bytes[69], bytes[70], bytes[71]]),
            version_valid_for_number: u32::from_be_bytes([
                                                         bytes[92], bytes[93], bytes[94], bytes[95],
            ]),
            version: u32::from_be_bytes([bytes[96], bytes[97], bytes[98], bytes[99]]),
        })
    }
}

pub fn get_number_of_tables(page_header: [u8;12]) -> u16 {
    u16::from_be_bytes([page_header[3], page_header[4]])
}

use std::fs::File;
use std::io::Read;

use anyhow::Result;

pub fn read_int(file: &mut File) -> Result<i64> {
    let mut value: i64 = 0;
    let mut byte = [0; 1];

    for i in 0..9 {
        file.read_exact(&mut byte)?;
        let byte = byte[0];

        if i == 8 {
            value = (value << 8) | byte as i64;
            return Ok(value);
        } else {
            value = (value << 7) | byte as i64;
            if byte < 0b1000_0000 {
                return Ok(value);
            }
        }
    }
    unreachable!("int reading failed");
}

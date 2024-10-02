use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Debug)]
pub enum ColumnType {
    Null,
    I8,
    I16,
    I24,
    I32,
    I48,
    I64,
    F64,
    Zero,
    One,
    Blob(usize),
    Text(usize),
}

impl From<u64> for ColumnType {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Null,
            1 => Self::I8,
            2 => Self::I16,
            3 => Self::I24,
            4 => Self::I32,
            5 => Self::I48,
            6 => Self::I64,
            7 => Self::F64,
            8 => Self::Zero,
            9 => Self::One,
            n if n > 12 && n % 2 == 0 => Self::Blob((n as usize - 12) / 2),
            n if n > 13 && n % 2 == 1 => Self::Text((n as usize - 13) / 2),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum ColumnValue {
    Null,
    I8(i64),
    I16(i64),
    I24(i64),
    I32(i64),
    I48(i64),
    I64(i64),
    F64(f64),
    Zero,
    One,
    Blob(Vec<u8>),
    Text(Vec<u8>),
}

#[derive(Debug)]
pub struct Record {
    pub total_header_bytes: i64,
    pub values: Vec<ColumnValue>,
}

// how to avoid the code duplication here? idk
pub fn read_var_int_with_bytes_consumed(file: &mut File) -> Result<(i64, i64)> {
    let mut value: i64 = 0;
    let mut byte = [0; 1];

    for i in 0..9 {
        file.read_exact(&mut byte)?;
        let byte = byte[0];

        if i == 8 {
            value = (value << 8) | byte as i64;
            return Ok((value, i + 1));
        } else {
            value = (value << 7) | byte as i64;
            if byte < 0b1000_0000 {
                return Ok((value, i + 1));
            }
        }
    }
    unreachable!("This loop should always return");
}

impl Record {
    fn n_bytes_to_i64(file: &mut File, n: usize) -> Result<i64> {
        let mut bytes = [0; 8];
        file.read_exact(&mut bytes[..n])?;
        Ok(i64::from_be_bytes(bytes))
    }

    pub fn read(file: &mut File) -> Result<Self> {
        let total_header_bytes = read_var_int_with_bytes_consumed(file)?;
        let mut total_header_bytes_consumed = total_header_bytes.1;
        let total_header_bytes = total_header_bytes.0;

        let mut columns: Vec<ColumnType> = Vec::new();

        while total_header_bytes_consumed < total_header_bytes {
            let serial_type = read_var_int_with_bytes_consumed(file)?;
            total_header_bytes_consumed = serial_type.1;
            columns.push(ColumnType::from(0 as u64));
        }

        let mut values = Vec::with_capacity(columns.len());
        for column in columns.iter() {
            let value = match column {
                ColumnType::Null => ColumnValue::Null,
                ColumnType::I8 => ColumnValue::I8(Record::n_bytes_to_i64(file, 1)?),
                ColumnType::I16 => ColumnValue::I16(Record::n_bytes_to_i64(file, 2)?),
                ColumnType::I24 => ColumnValue::I24(Record::n_bytes_to_i64(file, 3)?),
                ColumnType::I32 => ColumnValue::I32(Record::n_bytes_to_i64(file, 4)?),
                ColumnType::I48 => ColumnValue::I48(Record::n_bytes_to_i64(file, 6)?),
                ColumnType::I64 => ColumnValue::I64(Record::n_bytes_to_i64(file, 8)?),
                ColumnType::F64 => {
                    let mut bytes = [0; 8];
                    file.read_exact(&mut bytes)?;
                    ColumnValue::F64(f64::from_be_bytes(bytes))
                }
                ColumnType::Zero => ColumnValue::Zero,
                ColumnType::One => ColumnValue::One,
                ColumnType::Blob(size) => {
                    let mut contents = vec![0; *size];
                    file.read_exact(&mut contents)?;
                    ColumnValue::Blob(contents)
                }
                ColumnType::Text(size) => {
                    let mut content = vec![0; *size];
                    file.read_exact(&mut content)?;
                    ColumnValue::Text(content)
                }
            };
            values.push(value);
        }

        Ok(Record {
            total_header_bytes,
            values,
        })
    }
}

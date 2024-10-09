use std::io;
use std::io::Read;

use ciborium::Value;

#[derive(Default)]
pub struct Col {
    /// The length of the column. -1 means NULL.
    pub length: i32,

    pub raw: Vec<u8>,
}

impl Col {
    pub fn from_rdr<R>(rdr: &mut R) -> Result<Self, io::Error>
    where
        R: Read,
    {
        let mut buf: [u8; 4] = [0; 4];
        rdr.read_exact(&mut buf)?;
        let length: i32 = i32::from_be_bytes(buf);
        match length {
            -1 => Ok(Self {
                length,
                raw: vec![],
            }),
            _ => {
                let mut raw: Vec<u8> = vec![0; length as usize];
                let buf: &mut [u8] = &mut raw;
                rdr.read_exact(buf)?;
                Ok(Self { length, raw })
            }
        }
    }
}

impl Col {
    pub fn into_cbor_value(self) -> Value {
        match self.length {
            -1 => Value::Null,
            _ => Value::Bytes(self.raw),
        }
    }
}

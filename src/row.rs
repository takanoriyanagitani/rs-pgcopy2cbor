use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use ciborium::ser::Error as SerErr;
use ciborium::Value;

use crate::col::Col;
use crate::hdr::Header;

#[derive(Default)]
pub struct Row {
    pub cols: Vec<Col>,
}

impl Row {
    pub fn from_rdr<R>(rdr: &mut R) -> Result<Option<Self>, io::Error>
    where
        R: Read,
    {
        let mut buf: [u8; 2] = [0; 2];
        rdr.read_exact(&mut buf)?;
        let fcnt: i16 = i16::from_be_bytes(buf);
        match fcnt {
            -1 => Ok(None),
            _ => {
                let mut row: Row = Row {
                    cols: Vec::with_capacity(fcnt as usize),
                };
                for _ in 0..fcnt {
                    let col: Col = Col::from_rdr(rdr)?;
                    row.cols.push(col);
                }
                Ok(Some(row))
            }
        }
    }
}

impl Row {
    pub fn into_cbor_values(self) -> Vec<Value> {
        self.cols
            .into_iter()
            .map(|c: Col| c.into_cbor_value())
            .collect()
    }

    pub fn into_cbor_value(self) -> Value {
        let vals: Vec<Value> = self.into_cbor_values();
        Value::Array(vals)
    }
}

impl Row {
    pub fn into_writer<W>(self, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        let val: Value = self.into_cbor_value();
        ciborium::into_writer(&val, wtr).map_err(|e| match e {
            SerErr::Io(ie) => ie,
            SerErr::Value(s) => io::Error::other(s),
        })
    }
}

pub fn rdr2wtr<R, W>(rdr: &mut R, wtr: &mut W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    Header::from_rdr_no_validation(rdr)?;
    loop {
        let orow: Option<Row> = Row::from_rdr(rdr)?;
        match orow {
            None => {
                wtr.flush()?;
                return Ok(());
            }
            Some(row) => {
                row.into_writer(wtr)?;
            }
        }
    }
}

pub fn stdin2stdout() -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let mut br = BufReader::new(il);

    let o = io::stdout();
    let mut ol = o.lock();
    {
        let mut bw = BufWriter::new(ol.by_ref());
        rdr2wtr(&mut br, &mut bw)?;
        bw.flush()?;
    }
    ol.flush()?;
    Ok(())
}

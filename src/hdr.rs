use std::io;
use std::io::Read;

pub struct Sig {
    pub raw: [u8; 11],
}

impl Sig {
    pub fn from_rdr_no_validation<R>(rdr: &mut R) -> Result<Self, io::Error>
    where
        R: Read,
    {
        let mut s = Self { raw: [0; 11] };
        rdr.read_exact(&mut s.raw)?;
        Ok(s)
    }
}

/// Flags field.
pub struct Flags {
    pub raw: u32,
}

impl Flags {
    pub fn from_rdr_no_validation<R>(rdr: &mut R) -> Result<Self, io::Error>
    where
        R: Read,
    {
        let mut buf: [u8; 4] = [0; 4];
        rdr.read_exact(&mut buf)?;
        Ok(Self {
            raw: u32::from_be_bytes(buf),
        })
    }
}

/// Header extension area length.
pub struct Ext {
    pub raw: u32,
}

impl Ext {
    pub fn from_rdr_no_validation<R>(rdr: &mut R) -> Result<Self, io::Error>
    where
        R: Read,
    {
        let mut buf: [u8; 4] = [0; 4];
        rdr.read_exact(&mut buf)?;
        Ok(Self {
            raw: u32::from_be_bytes(buf),
        })
    }
}

pub struct Header {
    pub sig: Sig,
    pub flags: Flags,
    pub ext: Ext,
}

impl Header {
    pub fn from_rdr_no_validation<R>(rdr: &mut R) -> Result<Self, io::Error>
    where
        R: Read,
    {
        let sig: Sig = Sig::from_rdr_no_validation(rdr)?;
        let flags: Flags = Flags::from_rdr_no_validation(rdr)?;
        let ext: Ext = Ext::from_rdr_no_validation(rdr)?;
        Ok(Self { sig, flags, ext })
    }
}

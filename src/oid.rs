pub const GIT_OID_RAWSZ: usize = 20;

use sha1::Sha1;
use std::fmt;
use std::io::BufRead;
use std::ops::Deref;

use crate::error::Result;

#[derive(Clone, Copy)]
pub struct Oid {
    id: [u8; GIT_OID_RAWSZ],
}

impl Oid {
    pub fn new(id: [u8; GIT_OID_RAWSZ]) -> Self {
        Self { id }
    }

    pub fn from_hex<T: AsRef<[u8]>>(data: T) -> Self {
        let mut id = [0u8; GIT_OID_RAWSZ];
        id.copy_from_slice(&hex::decode(data).unwrap());
        Oid::new(id)
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Result<Self> {
        let mut id = [0u8; GIT_OID_RAWSZ];
        reader.read_exact(&mut id)?;
        Ok(Self { id })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.id
    }

    pub fn hex(&self) -> String {
        hex::encode(self.id)
    }
}

impl fmt::Debug for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hex())
    }
}

impl Deref for Oid {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

use std::fmt;
use std::io::BufRead;
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::blob::Blob;
use crate::error::Result;
use crate::oid::Oid;

pub struct IndexHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub num_entries: u32,
}

#[derive(Debug)]
pub struct IndexTime {
    pub seconds: i32,
    pub nanoseconds: u32,
}

pub struct IndexEntry {
    pub ctime: IndexTime,
    pub mtime: IndexTime,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub id: Oid,
    pub flags: u16,
    pub flags_extended: u16,
    pub path: Vec<u8>,
}

#[derive(Debug)]
pub struct Index {
    header: IndexHeader,
    entries: Vec<IndexEntry>,
}

impl IndexHeader {
    pub fn new() -> Self {
        Self {
            magic: *b"DIRC",
            version: 2,
            num_entries: 0,
        }
    }

    pub fn increment_entries(&mut self) {
        self.num_entries += 1
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Result<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).unwrap();
        let version = reader.read_u32::<BigEndian>()?;
        let num_entries = reader.read_u32::<BigEndian>()?;
        Ok(Self {
            magic,
            version,
            num_entries,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write(&self.magic)?;
        writer.write_u32::<BigEndian>(self.version)?;
        writer.write_u32::<BigEndian>(self.num_entries)?;
        Ok(())
    }
}

impl IndexTime {
    pub fn new(seconds: i32, nanoseconds: u32) -> Self {
        Self {
            seconds,
            nanoseconds,
        }
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Result<Self> {
        let seconds = reader.read_i32::<BigEndian>()?;
        let nanoseconds = reader.read_u32::<BigEndian>()?;
        Ok(Self {
            seconds,
            nanoseconds,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_i32::<BigEndian>(self.seconds)?;
        writer.write_u32::<BigEndian>(self.nanoseconds)?;
        Ok(())
    }
}

impl IndexEntry {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        // TODO: flags and flags_extended
        // let metadata = std::fs::metadata(&path).unwrap();
        // let ctime = IndexTime::new(metadata.ctime() as i32, metadata.ctime_nsec() as u32);
        // let mtime = IndexTime::new(metadata.mtime() as i32, metadata.mtime_nsec() as u32);
        // let blob = Blob::from_path(&path);
        // let path: Vec<u8> = path.as_ref().to_str().unwrap().bytes().collect();
        // let flags = path.len() as u16 & 0x0fff;
        // Self {
        //     ctime: ctime,
        //     mtime: mtime,
        //     dev: metadata.dev() as u32,
        //     ino: metadata.ino() as u32,
        //     mode: metadata.mode(),
        //     uid: metadata.uid(),
        //     gid: metadata.gid(),
        //     size: metadata.size() as u32,
        //     id: blob.id,
        //     flags: flags,
        //     flags_extended: 0,
        //     path: path,
        // }
        unimplemented!()
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Result<Self> {
        let ctime = IndexTime::from_reader(&mut reader)?;
        let mtime = IndexTime::from_reader(&mut reader)?;
        let dev = reader.read_u32::<BigEndian>()?;
        let ino = reader.read_u32::<BigEndian>()?;
        let mode = reader.read_u32::<BigEndian>()?;
        let uid = reader.read_u32::<BigEndian>()?;
        let gid = reader.read_u32::<BigEndian>()?;
        let size = reader.read_u32::<BigEndian>()?;
        let id = Oid::from_reader(&mut reader)?;
        let flags = reader.read_u16::<BigEndian>()?;
        // let flags_extended = read_u16(&mut reader);
        let flags_extended = 0u16;
        let name_len = flags & 0x0fff;
        let mut path = vec![0; name_len as usize];
        reader.read_exact(&mut path)?;
        let r = (name_len + 20 + 2) % 8;
        let remain = if r == 0 { 8 } else { 8 - r };
        let mut remain = reader.take(remain as u64);
        remain.read_to_end(&mut Vec::new())?;
        Ok(Self {
            ctime: ctime,
            mtime: mtime,
            dev: dev,
            ino: ino,
            mode: mode,
            uid: uid,
            gid: gid,
            size: size,
            id: id,
            flags: flags,
            flags_extended: flags_extended,
            path: path,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        self.ctime.write(&mut writer)?;
        self.mtime.write(&mut writer)?;
        writer.write_u32::<BigEndian>(self.dev)?;
        writer.write_u32::<BigEndian>(self.ino)?;
        writer.write_u32::<BigEndian>(self.mode)?;
        writer.write_u32::<BigEndian>(self.uid)?;
        writer.write_u32::<BigEndian>(self.gid)?;
        writer.write_u32::<BigEndian>(self.size)?;
        writer.write(self.id.as_bytes())?;
        writer.write_u16::<BigEndian>(self.flags)?;
        writer.write(&self.path)?;
        let name_len = self.path.len();
        let r = (name_len + 20 + 2) % 8;
        let padding = if r == 0 { 8 } else { 8 - r };
        writer.write(&vec![0u8; padding])?;
        Ok(())
    }
}

impl Index {
    pub fn new() -> Self {
        Self {
            header: IndexHeader::new(),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, index_entry: IndexEntry) {
        self.header.increment_entries();
        self.entries.push(index_entry);
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Result<Self> {
        let header = IndexHeader::from_reader(&mut reader)?;
        let mut entries = Vec::new();
        for _ in 0..(header.num_entries) {
            entries.push(IndexEntry::from_reader(&mut reader)?);
        }
        Ok(Self { header, entries })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        self.header.write(&mut writer)?;
        for entry in &self.entries {
            entry.write(&mut writer)?;
        }
        Ok(())
    }
}

impl fmt::Debug for IndexHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IndexHeader {{ magic: {}, version: {}, num_entries: {} }}",
            std::str::from_utf8(&self.magic).unwrap(),
            self.version,
            self.num_entries
        )
    }
}

impl fmt::Debug for IndexEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IndexEntry {{ ctime: {:?}, mtime: {:?}, dev: {}, ino: {}, mode: {}, uid: {}, gid: {}, size: {}, id: {:?}, flags: {}, flags_extended: {}, path: {} }}",
            self.ctime, self.mtime, self.dev, self.ino, self.mode, self.uid, self.gid, self.size, self.id, self.flags, self.flags_extended, std::str::from_utf8(&self.path).unwrap()
        )
    }
}

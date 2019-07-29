use std::fmt;
use std::io::BufRead;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::blob::Blob;
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

    pub fn from_reader<B: BufRead>(mut reader: B) -> Self {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).unwrap();
        let version = read_u32(&mut reader);
        let num_entries = read_u32(&mut reader);
        Self {
            magic,
            version,
            num_entries,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.magic);
        bytes.extend_from_slice(&self.version.to_be_bytes());
        bytes.extend_from_slice(&self.num_entries.to_be_bytes());
        bytes
    }
}

impl IndexTime {
    pub fn new(seconds: i32, nanoseconds: u32) -> Self {
        Self {
            seconds,
            nanoseconds,
        }
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Self {
        let seconds = read_i32(&mut reader);
        let nanoseconds = read_u32(&mut reader);
        Self {
            seconds,
            nanoseconds,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.seconds.to_be_bytes());
        bytes.extend_from_slice(&self.nanoseconds.to_be_bytes());
        bytes
    }
}

impl IndexEntry {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        // TODO: flags and flags_extended
        let metadata = std::fs::metadata(&path).unwrap();
        let ctime = IndexTime::new(metadata.ctime() as i32, metadata.ctime_nsec() as u32);
        let mtime = IndexTime::new(metadata.mtime() as i32, metadata.mtime_nsec() as u32);
        let blob = Blob::from_path(&path);
        let path: Vec<u8> = path.as_ref().to_str().unwrap().bytes().collect();
        let flags = path.len() as u16 & 0x0fff;
        Self {
            ctime: ctime,
            mtime: mtime,
            dev: metadata.dev() as u32,
            ino: metadata.ino() as u32,
            mode: metadata.mode(),
            uid: metadata.uid(),
            gid: metadata.gid(),
            size: metadata.size() as u32,
            id: blob.id,
            flags: flags,
            flags_extended: 0,
            path: path,
        }
    }

    pub fn from_reader<B: BufRead>(mut reader: B) -> Self {
        let ctime = IndexTime::from_reader(&mut reader);
        let mtime = IndexTime::from_reader(&mut reader);
        let dev = read_u32(&mut reader);
        let ino = read_u32(&mut reader);
        let mode = read_u32(&mut reader);
        let uid = read_u32(&mut reader);
        let gid = read_u32(&mut reader);
        let size = read_u32(&mut reader);
        let id = Oid::from_reader(&mut reader);
        let flags = read_u16(&mut reader);
        // let flags_extended = read_u16(&mut reader);
        let flags_extended = 0u16;
        let name_len = flags & 0x0fff;
        let mut path = vec![0; name_len as usize];
        reader.read_exact(&mut path);
        let r = (name_len + 20 + 2) % 8;
        let remain = if r == 0 { 8 } else { 8 - r };
        let mut remain = reader.take(remain as u64);
        remain.read_to_end(&mut Vec::new());
        Self {
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
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.ctime.bytes());
        bytes.append(&mut self.mtime.bytes());
        bytes.extend_from_slice(&self.dev.to_be_bytes());
        bytes.extend_from_slice(&self.ino.to_be_bytes());
        bytes.extend_from_slice(&self.mode.to_be_bytes());
        bytes.extend_from_slice(&self.uid.to_be_bytes());
        bytes.extend_from_slice(&self.gid.to_be_bytes());
        bytes.extend_from_slice(&self.size.to_be_bytes());
        bytes.extend_from_slice(self.id.as_bytes());
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&self.path);
        let name_len = self.path.len();
        let r = (name_len + 20 + 2) % 8;
        let padding = if r == 0 { 8 } else { 8 - r };
        bytes.append(&mut vec![0u8; padding]);
        bytes
    }
}

fn read_i32<B: BufRead>(mut reader: B) -> i32 {
    let mut tmp = [0u8; 4];
    reader.read_exact(&mut tmp);
    i32::from_be_bytes(tmp)
}

fn read_u32<B: BufRead>(mut reader: B) -> u32 {
    let mut tmp = [0u8; 4];
    reader.read_exact(&mut tmp);
    u32::from_be_bytes(tmp)
}

fn read_u16<B: BufRead>(mut reader: B) -> u16 {
    let mut tmp = [0u8; 2];
    reader.read_exact(&mut tmp);
    u16::from_be_bytes(tmp)
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

    pub fn from_reader<B: BufRead>(mut reader: B) -> Self {
        let header = IndexHeader::from_reader(&mut reader);
        let mut entries = Vec::new();
        for _ in 0..(header.num_entries) {
            entries.push(IndexEntry::from_reader(&mut reader));
        }
        Self { header, entries }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.header.bytes());
        for entry in &self.entries {
            bytes.append(&mut entry.bytes());
        }
        bytes
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

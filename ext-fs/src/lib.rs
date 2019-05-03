use std::fs as stdfs;
use std::path::Path;
use std::io::{Read, Write};

pub fn read_to_string<P: AsRef<Path>>(path: P) -> String {
    stdfs::File::open(path).map(|mut fp| {
        let mut buffer = String::new();
        fp.read_to_string(&mut buffer).unwrap();
        fp.flush().unwrap();
        buffer
    }).unwrap()
}

pub fn read_to_end<P: AsRef<Path>>(path: P) -> Vec<u8> {
    stdfs::File::open(path).map(|mut fp| {
        let mut buffer = Vec::new();
        fp.read_to_end(&mut buffer).unwrap();
        fp.flush().unwrap();
        buffer
    }).unwrap()
}

pub fn create_write_all<P: AsRef<Path>>(path: P, mut buf: &[u8]) {
    stdfs::File::create(path).map(|mut fp| {
        fp.write_all(&mut buf).unwrap();
        fp.flush().unwrap();
    }).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

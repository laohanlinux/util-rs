#![feature(result_map_or_else)]

use std::fs as stdfs;
use std::io::{Read, Write};
use std::path::Path;

pub fn read_to_string<P: AsRef<Path>>(path: P) -> String {
    stdfs::File::open(path)
        .map(|mut fp| {
            let mut buffer = String::new();
            fp.read_to_string(&mut buffer).unwrap();
            fp.flush().unwrap();
            buffer
        })
        .unwrap()
}

pub fn read_to_end<P: AsRef<Path>>(path: P) -> Vec<u8> {
    stdfs::File::open(path)
        .map(|mut fp| {
            let mut buffer = Vec::new();
            fp.read_to_end(&mut buffer).unwrap();
            fp.flush().unwrap();
            buffer
        })
        .unwrap()
}

pub fn create_write_all<P: AsRef<Path>>(path: P, mut buf: &[u8]) {
    stdfs::File::create(path)
        .map(|mut fp| {
            fp.write_all(&mut buf).unwrap();
            fp.flush().unwrap();
        })
        .unwrap()
}

pub fn exist_file<P: AsRef<Path>>(path: P) -> bool {
    stdfs::metadata(path).map_or_else(|_| false, |meta| meta.is_file())
}

pub fn exist_dir<P: AsRef<Path>>(path: P) -> bool {
    stdfs::metadata(path).map_or_else(|_| false, |meta| meta.is_dir())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exist_dir() {
        let got = crate::exist_dir("/tmp");
        assert!(got);

        let got = crate::exist_dir("/tmp1");
        assert!(!got);
    }

    #[test]
    fn exist_file() {
        use crate::create_write_all;
        create_write_all("/tmp/a", &vec![]);
        let got = crate::exist_file("/tmp/a");
        assert!(got);

        let got = crate::exist_file("/tmp/a1");
        assert!(!got);

        crate::stdfs::remove_dir_all("/tmp/a");
    }
}

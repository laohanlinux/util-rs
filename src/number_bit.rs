// 0xf => 1111, 1111, 1111, 1111,
pub fn u32_big_to_bin(x: u32) -> [u8; 4] {
    let b1 = ((x >> 24) & 0xff) as u8;
    let b2 = ((x >> 16) & 0xff) as u8;
    let b3 = ((x >> 8) & 0xff) as u8;
    let b4 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

pub fn u32_big_fill_buf(x: u32, buf: &mut [u8]) {
    assert!(buf.len() == 4);
    buf[0] = ((x >> 24) & 0xff) as u8;
    buf[1] = ((x >> 16) & 0xff) as u8;
    buf[2] = ((x >> 8) & 0xff) as u8;
    buf[3] = (x & 0xff) as u8;
}

pub fn bin_big_to_u32(x: &[u8]) -> u32 {
    assert!(x.len() == 4);
    let b1 = (x[0] as u32) << 24;
    let b2 = (x[1] as u32) << 16;
    let b3 = (x[2] as u32) << 8;
    let b4 = x[3] as u32;
    b1 + b2 + b3 + b4
}

// 0xf => 1111, 1111, 1111,1111,
pub fn u64_big_to_bin(x: u64) -> [u8; 8] {
    let b1 = ((x >> 56) & 0xffff) as u8;
    let b2 = ((x >> 48) & 0xffff) as u8;
    let b3 = ((x >> 40) & 0xffff) as u8;
    let b4 = ((x >> 32) & 0xffff) as u8;
    let b5 = ((x >> 24) & 0xffff) as u8;
    let b6 = ((x >> 16) & 0xffff) as u8;
    let b7 = ((x >> 8) & 0xffff) as u8;
    let b8 = (x & 0xffff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8];
}

pub fn u64_big_fill_buf(x: u64, buf: &mut [u8]) {
    assert!(buf.len() == 8);
    buf[0] = ((x >> 56) & 0xffff) as u8;
    buf[1] = ((x >> 48) & 0xffff) as u8;
    buf[2] = ((x >> 40) & 0xffff) as u8;
    buf[3] = ((x >> 32) & 0xffff) as u8;
    buf[4] = ((x >> 24) & 0xffff) as u8;
    buf[5] = ((x >> 16) & 0xffff) as u8;
    buf[6] = ((x >> 8) & 0xffff) as u8;
    buf[7] = (x & 0xffff) as u8;
}

pub fn bin_big_to_u64(x: &[u8]) -> u64 {
    assert!(x.len() == 8);
    let b1 = (x[0] as u64) << 56;
    let b2 = (x[1] as u64) << 48;
    let b3 = (x[2] as u64) << 40;
    let b4 = (x[3] as u64) << 32;
    let b5 = (x[4] as u64) << 24;
    let b6 = (x[5] as u64) << 16;
    let b7 = (x[6] as u64) << 8;
    let b8 = x[7] as u64;
    b1 + b2 + b3 + b4 + b5 + b6 + b7 + b8
}

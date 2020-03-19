// adapted from: https://github.com/Cuyler36/HorizonSummer/blob/master/HorizonSummer/Checksums.cs

use byteorder::{ByteOrder, LittleEndian};
use std::num::Wrapping;

pub fn murmur32(data: &[u8]) -> u32 {
    let mut hash = Wrapping(0u32);
    let mut position = 0usize;
    let len = data.len();

    while position as isize <= len as isize - 4 {
        let val = LittleEndian::read_u32(&data[position..]);
        hash ^= murmur32_scramble(Wrapping(val));
        hash = (hash >> 19) | (hash << 13);
        hash = hash * Wrapping(5) + Wrapping(0xE654_6B64);

        position += 4;
    }

    let remainder = len - position;
    if remainder > 0 {
        let mut tempbuffer = [0u8; 4];
        tempbuffer[0..remainder].copy_from_slice(&data[position..]);
        let val = LittleEndian::read_u32(&tempbuffer) >> (8 * (4 - remainder));
        hash ^= murmur32_scramble(Wrapping(val));
    }

    hash ^= Wrapping(len as u32);
    hash ^= hash >> 16;
    hash *= Wrapping(0x85EB_CA6B);
    hash ^= hash >> 13;
    hash *= Wrapping(0xC2B2_AE35);
    hash ^= hash >> 16;

    hash.0
}

#[inline]
fn murmur32_scramble(k: Wrapping<u32>) -> Wrapping<u32> {
    ((k * Wrapping(0x16A8_8000)) | (k * Wrapping(0xCC9E_2D51)) >> 17) * Wrapping(0x1B87_3593)
}

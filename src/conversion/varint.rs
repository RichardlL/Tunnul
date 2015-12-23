// Tunnul - Minecraft server
// Copyright 2015 Richard Lettich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
// IT IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
//

// Calulates size  of varint what how to decode
// transfomed input flips negatives, which are then fliped back, to use less data
// the u8 depends on if it was flipped at first
fn var_int_flip(src: i32) -> (i32, u8) {
    match (src & (0x1 << 31i32)) as u32 {
        0x80000000u32 => ((src ^ ((0x1i32 << 31) >> 31), 0xFFu8)),
        _ => (src, 0x80u8),
    }

}
// bytes is number of bytes it uses
pub fn var_int_size(transformed: i32) -> usize {
    (((((32 - transformed.leading_zeros()) as usize) * 9 + 7) / 8) + 7) / 8
}

pub fn to(src: i32) -> Vec<u8> {
    let (transform, y) = var_int_flip(src);
    let bytes = var_int_size(transform);
    let mut result = (0..bytes)
                         .map(|i| y ^ ((transform >> (7 * i)) as u8) & 0x7F)
                         .collect::<Vec<u8>>();
    result[bytes - 1] ^= 0x80u8;
    result
}
pub fn write_to(src: i32, mut vector: &mut Vec<u8>) {
    let (transform, y) = var_int_flip(src);
    let start = vector.len();
    let end = start + var_int_size(transform);
    unsafe { vector.set_len(end) };
    for (i, val) in (start..end).enumerate() {
        vector[val] = y ^ (((transform >> (7 * i)) as u8) & 0x7F);
    }
    vector[end - 1] ^= 0x80u8;
}

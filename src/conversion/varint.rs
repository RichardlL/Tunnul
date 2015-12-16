/* Tunnul - Minecraft server
 * Copyright 2015 Richard Lettich
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
 * IT IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
 */

pub fn to(src: &mut i32) -> Vec<u8> {
        let (transform, y) = match (*src  & (0x1 << 31i32)) as u32 {
                0x80000000u32 => ((src.clone() ^( (0x1i32 << 31) >> 31), 0xFFu8)),
                _ => (src.clone(), 0x80u8),
        };
        
        let bytes:usize = (((((32 - transform.leading_zeros()) as usize)* 9 + 7) / 8) + 7) / 8;
        let mut result: Vec<u8> = vec![0;8];
        for i in 0..bytes {
                result[i] = y ^ (((transform >> (7*i))as u8) & 0x7F);
        }
        result[bytes - 1] ^= 0x80u8; //flips last digit show end of varint
        result.truncate(bytes);
        result
}
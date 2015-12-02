/* Tunul - Minecraft server
 * Copyright 2015 Richard Lettich
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
 * TUNUL IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
 */
use std::i32;
pub fn from(src_array:&[u8]) -> (i32, usize) {
        let mut result:i32 = 0;
        let mut vi_size:usize = 0;
        loop {
                result |= ((src_array[vi_size] & 0x7Fu8 ) as i32)  << (7 * vi_size) ;
                if src_array[vi_size] & 0x80u8 == 0 {
                        break;
                }
                vi_size += 1;

        }
        vi_size +=1;
        result |= ((result & 0x40) << 25) >> (31-(7 * vi_size));
        (result, vi_size)
}

fn from_long(src_array:&[u8]) -> (i64, usize) {
        let mut result:i64 = 0;
        let mut vi_size:usize = 0;
        loop {
                result |= ((src_array[vi_size] & 0x7Fu8 ) as i64)  << (57 - (7 * vi_size)) ;
                vi_size += 1;
                if src_array[vi_size] & 0x80u8 == 0 {
                        break;
                }
        }
        (result >> (64-(7*vi_size)), vi_size)
}

pub fn to(src: i32) -> Vec<u8> {
        let plus_num = |x:i32 , y| -> Vec<u8> {
                let bytes:usize = (((((32 - x.leading_zeros()) as usize)* 9 + 7) / 8) + 7) / 8;
                let mut result: Vec<u8> = vec![0; bytes];
                for i in 0..bytes {
                        result[i] = y ^ (((x >> (7*i))as u8) & 0x7F);
                }
                result[bytes - 1] ^= 0x80u8; //flips last digit show end of varint
                result
        };
        // This flips the digits around on negative numbers
        // negative are encoded asreally tall positives such as
        // so that adding them would be equivlent
        // y = 1111111 (-1) + 1  
        // y would over flow to zero, but for a 64 bit, 1 would use all 64 bits, so this flips it,
        // and uses the minimum number of bytes (i.e. first 0x1 found instead of 0)
        // and then flips it back
        match (src  & (0x1 << 31i32)) as u32 {
                0x80000000u32  => plus_num((src ^( (0x1i32 << 31) >> 31)),0xFFu8),
                _  => plus_num(src,0x80u8),
        }
}
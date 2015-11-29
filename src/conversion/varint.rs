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
                println!("source: {}", src_array[vi_size]);
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

fn to(src: i64) -> Box<[u8]> {
        let plus_num = |x, y| -> Box<[u8]> {
                let bytes:usize = ((64 - (src.leading_zeros() as usize)) * 9  + 7) / 8;
                let mut result: Vec<u8> = vec![0; bytes];;
                for i in 0..bytes+1 {
                        result[(bytes-1)-i] = (y ^ ((x << i)as u8 & 0x7F));
                }
                result[10] &= 0x7F;
                result.into_boxed_slice()
        };
        match src  & 0x1 << 63i64 {
                0x100000000  => plus_num((src ^( 0x100000000i64 >> 63)),0xFFu8),
                _  => plus_num(src,0x80u8),
        }
}
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

use std::net::TcpStream;
use std::io::Read;
use std::i32;
pub fn read(src_array:&mut TcpStream) -> (i32, usize) {
        let mut result:i32 = 0;
        let mut vi_size:usize = 0;
        for byte in  src_array.bytes() {
                let byte = byte.unwrap();
                result |= ((byte  & 0x7Fu8)  as i32)  << (7 * vi_size);
                vi_size += 1;
                if (byte & 0x80u8) == 0 {
                        break;
                }
                result |= ((result & 0x40) << 25) >> (31-(7 * vi_size));
        }
        (result, vi_size)
}


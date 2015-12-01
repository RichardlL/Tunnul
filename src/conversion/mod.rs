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

pub mod itt;
pub mod varint;

use std::cell::RefCell;
//turns string to bytes, and prefixes with length as varint
pub fn to_string(src_string : String) -> [Vec<u8>;2] {
        let src_string  = src_string.into_bytes();
        let src_string_length = varint::to((src_string.len() as i32)); 
        [src_string_length, src_string]
}
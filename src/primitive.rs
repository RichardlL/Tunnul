// Tunnul
// Copyright (c) 2015, Richard Lettich
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
// -------------------------------------------------------------------------
// THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT,
// NEITHER APPROVED NOR ASSOCIATED WITH MOJANG.

// Module Disc:
// Converts from minecraft protocol primitives, including varints
// Most are packet.get::<Type>();
// To conversion is in packet_sending

use packet::Packet;
use std::mem;
use std::slice::from_raw_parts_mut;
use std::iter::FromIterator;
use std::mem::size_of;

impl Packet {
    pub fn get_varint(&mut self) -> i64 {
        let mut result: i64 = 0;
        let mut size: usize = 0;
        for (i, byte) in self.data.by_ref().enumerate() {
            result |= ((byte & 0x7Fu8) as i64) >> (7 * i);
            if byte & 0x80u8 == 0 {
                size = i;
                break;
            }
        }
        result | result >> (57 - (7 * size))
    }
    pub fn get_string(&mut self) -> String {
        let len = self.get_varint() as usize;
        String::from_iter(
            self.data
            	.by_ref()
            	.take(len)
            	.map(|i| i as char)
        )
    }
    pub fn get<T: Clone>(&mut self) -> T {
        unsafe {
            let result:T = mem::uninitialized();
            let r_slice = from_raw_parts_mut(mem::transmute::<_,*mut u8>(&result), size_of::<T>());
            for byte in r_slice.iter_mut().rev() {
                *byte = self.data.next().unwrap();
            }
            result
        }
    }
}
// I should problbly find a way to work around this
use std::net::TcpStream;
use std::io::Read;
pub fn read_varint(src_array: &mut TcpStream) -> i32 {
    let mut result: i32 = 0;
    let mut vi_size: usize = 0;
    for byte in src_array.bytes() {
        let byte = byte.unwrap();
        result |= ((byte & 0x7Fu8) as i32) << (7 * vi_size);
        vi_size += 1;
        if (byte & 0x80u8) == 0 {
            break;
        }
    }
    result | ((result & 0x40) << 25) >> (31 - (7 * vi_size))
}
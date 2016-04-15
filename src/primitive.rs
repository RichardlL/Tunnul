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
// 3. The name of the author may not be used to endose or promote products
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
use std::mem::{ size_of, transmute, zeroed};

impl Packet {
    pub fn get_varint(&mut self) -> i64 {
        self.data.get_varint()
    }
    pub fn get_string(&mut self) -> String {
        let len = self.get_varint() as usize;
        self.data
            .by_ref()
            .take(len)
            .map(|c| c as char)
            .collect::<String>()
    }
    
    pub fn getas<T: Clone>(&mut self) -> T {
        let mut result: [u8; 8] = [0;8];
        for (i, byte) in result[..size_of::<T>()].iter_mut().rev().zip(self.data.by_ref()) { 
            *i = byte;
        }
        unsafe { (*transmute::<_,&T>(&result)).clone() }
    }
}

pub trait VarInt { fn get_varint(&mut self) -> i64; }

use std::iter::Iterator;
// This is recieved Big edian, while most computers are little edian, so we write to it
// in reverse order The first bit of a byte (most significant) is 1 if there is additional
// bytes after words.  The rest of the byte (7 least signifacant, or & 0x7Fu8)
// are the actually number
impl<T: Iterator<Item = u8>> VarInt for T {
    fn get_varint(&mut self) -> i64 {
        let mut result = 0;
        let mut offset = 0;
        for byte in self.take(10) {
            result |= ((byte & 0x7Fu8) as i64) << offset;
            offset += 7;
            if byte & 0x80u8 == 0 {
                break;
            }
        }
        let shift = 57 - offset; //
        (result << shift) >> shift
        // This voodoo slides slides the left most bit to the end, then back. Rust auto extends signs
        // E.G. 0b1000 >> 3 yields 0b1111
    }
}


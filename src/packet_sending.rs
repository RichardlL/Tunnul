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
// --------------------------------------------------------------------------
// THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT,
// NEITHER APPROVED NOR ASSOCIATED WITH MOJANG.

// Do to the use of generics (soon, rust needs some features, see below)
// and macros, I feel the need to explain how this works,
// as I probably will wonder what this even does in a week.
//

// & .convert() method
// -- Converts Data as neccesary, (little to big edian),
// string types or does nothing for existing byte arrays
//
pub type Var32 = i32;
#[macro_use]
/// -  Takes A TcpStream, Packet ID, and 0 or more pieces of Data to send to the client
/// -  Finds lenth using .size() method for each type, then formats packet, allocating a large Vec first
macro_rules! Send {
    { $stream:expr, $packet_id:expr, $( $data:expr ),* } => { {
            use packet_sending::{write_varint, CanSend};
            use std::io::Write;
            use std::net::TcpStream;
            let mut packet_size = 1; // Packet id is One byte
            $(
                packet_size += ($data).get_size();
            )*
            let mut packet: Vec<u8> = Vec::new();
            write_varint(packet_size as i32, &mut packet);
            packet.reserve(packet_size);
            packet.push($packet_id as u8);
            $(
                $data.convert_into(&mut packet);
            )*
            let _ = TcpStream::write(&mut $stream, &packet[..]);
        } };
}
/// Crude Generic reimplementation to workout lack of Specialization, see
/// https://github.com/rust-lang/rfcs/issues/1053
/// Use Implsend!(Type, sizeoftype);
macro_rules! ImplSend {
    ( $t:ty, $size:expr ) => {
        impl CanSend for $t {
            fn get_size(&self) -> usize {
                $size
            }
            fn convert_into(&self, mut packet: &mut Vec<u8>) {
                reverse_and_write(self, $size, &mut packet);
            }
        }
    }
}
use std::mem;
use std::any::Any;
use std::slice::from_raw_parts;
/// Trait of Primitive types that can be sent
pub trait CanSend {
    fn get_size(&self) -> usize;
    fn convert_into(&self, mut packet: &mut Vec<u8>);
}
impl CanSend for str {
    fn get_size(&self) -> usize {
        self.len() + var_int_size(self.len() as i32) as usize
    }
    fn convert_into(&self, mut packet: &mut Vec<u8>) {
        write_varint(self.len() as i32, packet);
        packet.extend_from_slice(self.as_bytes());
    }
}
use struct_types::Location;
impl CanSend for Location {
    fn get_size(&self) -> usize {
        24
    }
    fn convert_into(&self,mut packet: &mut Vec<u8>) {
        self.x.convert_into(packet);
        self.y.convert_into(packet);
        self.z.convert_into(packet);
    }
}
// We have to do this explicitly, as we check If its a varint
// type aliases can fit what they are defined as, but not the other way around
impl CanSend for i32 {
    fn get_size(&self) -> usize {
        let new = self as &Any;
        if new.is::<Var32>() {
            var_int_size(*self) as usize
        } else {
            4
        }
    }
    fn convert_into(&self, mut packet: &mut Vec<u8>) {
        let new = self as &Any;
        if new.is::<Var32>() {
            write_varint(*self, packet)
        } else {
            reverse_and_write(self, 4, packet)
        }
    }
}

fn reverse_and_write<T>(pointer: &T, size: usize, mut packet: &mut Vec<u8>) {
    unsafe {
        let raw: *const u8 = mem::transmute(pointer);
        let t_as_u8_slice = from_raw_parts(raw, size);
        packet.extend(t_as_u8_slice.iter().rev());
    }
}
pub fn write_varint(src: i32, mut vector: &mut Vec<u8>) {
    let (transform, xor) = match src.is_positive() {
        true => (src, 0x80u8),
        false => ((src ^ 0x7FFFFFFF, 0xFFu8)),
    };
    vector.extend(
        (0..var_int_size(transform))
        .rev()
        .map(|i| xor ^ (((transform >> (7 * i)) as u8) & 0x7F))
    );
    let tmp = vector.len() -1;
    vector[tmp] ^= 0x80u8;
}
ImplSend!(i64, 8);
ImplSend!(i16, 2);
// Due to varint, i32 is implement seperatly
ImplSend!(i8, 1);
ImplSend!(u64, 8);
ImplSend!(u32, 4);
ImplSend!(u16, 2);
ImplSend!(u8, 1);
ImplSend!(f64, 8);
ImplSend!(f32, 4);
pub fn var_int_size(transformed: i32) -> usize {
    match transformed {
        0...127 => 1,
        128...16384  => 2,
        16385...2097152 => 3,
        _ => 4,
    }
}


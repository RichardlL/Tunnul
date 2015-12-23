// Tunnul - Minecraft server
// Copyright 2015 Richard Lettich
//
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
// IT IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
//
// Do to the use of generics and macros, I feel the need to explain how this works,
// as I probably will wonder what this even does in a week.
//
// &  Send! Macro
// -  takes A TcpStream, Packet ID, and 0 or more pieces of Data to send to the client
// -  Finds lenth using .size() method for each type, as packets are formatted as
// - Lenth of Packet_ID + Data
// - Packet id
// - Data
// - Sends Packet Size
// - Sends Packet ID
// - Sends Data
// - This seems like a lot of bloat, but most of it will be compiled out (hopefully!) :)
//
// & .convert() method
// -- Converts Data as neccesary, (little to big edian),
// string types or does nothing for existing byte arrays
//
// We have to implement for each type, Since lack of type specialization
// See https://github.com/rust-lang/rfcs/issues/1053
//
// This can be done in less lines with downcast_ref/mut(), but this way is done at compile time
pub type Var32 = i32;
#[macro_use]
macro_rules! Send {
    { $stream:expr, $packet_id:expr, $( $data:expr ),* } => { {
            use packet_sending::CanSend;
            use std::io::Write;
            use conversion::varint;
            let mut packet_size = 1; // Packet id is One byte
            $(
                packet_size += ($data).get_size();
            )*
            let mut packet = varint::to(packet_size as i32);
            packet.reserve(packet_size);
            packet.push($packet_id as u8);
            $(
                $data.convert_into(&mut packet);
            )*
            let _ = (&mut $stream).write(&packet[..]);
        } };
}
macro_rules! ImplSend {
    ( $t:ty, $size:expr ) => {
        impl CanSend for $t {
            fn get_size(&self) -> usize {
                $size
            }
            fn convert_into(&mut self, mut packet: &mut Vec<u8>) {
                reverse_and_write(self, $size, &mut packet);
            }
        }
    }
}
use std::mem;
use conversion::varint;
use std::any::Any;
use std::slice::from_raw_parts_mut;
pub trait CanSend {
    fn get_size(&self) -> usize;
    fn convert_into(&mut self, mut packet: &mut Vec<u8>);
}
impl CanSend for String {
    fn get_size(&self) -> usize {
        self.len() + varint::var_int_size(self.len() as i32) as usize
    }
    fn convert_into(&mut self, mut packet: &mut Vec<u8>) {
        varint::write_to((self.len() as i32), packet);
        packet.extend_from_slice(self.as_bytes());
    }
}
// We have to do this explicitly, as we check If its a varint
// type aliases can fit what they are defined as, but not the other way around
impl CanSend for i32 {
    fn get_size(&self) -> usize {
        let new = self as &Any;
        if new.is::<Var32>() {
            varint::var_int_size(*self) as usize
        } else {
            4
        }
    }
    fn convert_into(&mut self, mut packet: &mut Vec<u8>) {
        let new = self as &Any;
        if new.is::<Var32>() {
            varint::write_to(*self, packet)
        } else {
            reverse_and_write(self, 4, packet)
        }
    }
}
impl CanSend for Vec<u8> {
    fn get_size(&self) -> usize {
        self.len()
    }
    fn convert_into(&mut self, mut packet: &mut Vec<u8>) {
        packet.extend(&*self);
    }
}
fn reverse_and_write<T>(pointer: &T, size: usize, mut packet: &mut Vec<u8>) {
    unsafe {
        let raw: *mut u8 = mem::transmute(pointer);
        let t_as_u8_slice = from_raw_parts_mut(raw, size);
        packet.extend(t_as_u8_slice.iter().rev());
    }
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

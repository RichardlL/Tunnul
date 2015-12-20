/*
 * Tunnul - Minecraft server
 * Copyright 2015 Richard Lettich
 *
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
 * IT IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
 */
/*
 Do to the use of generics and macros, I feel the need to explain how this works,
 as I probably will wonder what this even does in a week.

  &  Send! Macro  
    -  takes A TcpStream, Packet ID, and 0 or more pieces of Data to send to the client
      -  Finds lenth using .size() method for each type, as packets are formatted as
         - Lenth of Packet_ID + Data
         - Packet id
         - Data
      - Sends Packet Size
      - Sends Packet ID
      - Sends Data
      - This seems like a lot of bloat, but most of it will be compiled out (hopefully!) :)

  & .convert() method
    -- Converts Data as neccesary, (little to big edian), string types or does nothing for existing byte arrays

  This Probably reimplements Format!, but it seems more intuitive
*/
type Var32 = i32;
#[macro_use]
macro_rules! Send {
        { $stream:expr, $packet_id:expr, $( $data:expr ),* } => { {
                    use packet_sending::CanSend;
                    use std::io::Write;
                    use conversion::varint;
                    let mut packet_size = 1; // Packet id is One byte
                    $(
                        packet_size += (&$data).get_size();
                    )*
                    let mut packet = varint::to(&mut (packet_size as i32));
                    packet.reserve(packet_size);
                    packet.push($packet_id as u8);
                    $(
                        $data.convert_into(&mut packet);
                    )*
                    for i in &packet {
                        println!("i {}", i);
                    }
                    let _ = (&mut $stream).write(&packet[..]);
                    let _ = (&mut $stream).flush();
        } };
}
use std::mem;
use conversion::varint;
use std::any::Any;
use std::slice::from_raw_parts_mut;
pub trait CanSend {
    fn get_size(&self) -> usize;
    fn convert_into(&mut self, packet: &mut Vec<u8>);
}
impl<T: Any + 'static> CanSend for T {
    fn get_size(&self) -> usize {
        let from = self as &Any;
        if let Some(string) = from.downcast_ref::<String>() {
            string.len() + varint::to(&mut (string.len() as i32)).len()
       // } else if let Some(variable) = from.downcast_ref::<Var32>() {
       //     varint::to(&mut variable.clone()).len() as usize
        } else if let Some(vector) = from.downcast_ref::<Vec<u8>>() {
            vector.len()
        } else {
            mem::size_of::<T>()
        }
    }
    fn convert_into(&mut self, packet: &mut Vec<u8>) {
        let mut from = self as &mut Any;
        if let Some(string) = from.downcast_ref::<String>() {
            varint::write_to((string.len() as i32), packet);
            packet.extend_from_slice(string.as_bytes());
            return;
        //} else if let Some(variable) = from.downcast_ref::<Var32>() {
          //  varint::write_to(*variable, packet)
        } if let Some(vector) = from.downcast_ref::<Vec<u8>>() {
            packet.extend(vector);
            return;
        } unsafe {
            let raw: *mut u8 = mem::transmute(from.downcast_mut::<T>().unwrap());
            let t_as_u8_slice = from_raw_parts_mut(raw, mem::size_of::<T>());
            packet.extend(t_as_u8_slice.iter().rev());
        }
    }
}

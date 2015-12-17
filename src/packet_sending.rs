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

#[macro_use]
macro_rules! Send {
        { $stream:expr, $packet_id:expr, $( $data:expr ),* } => {
                {	use packet_sending::CanSend;
                        use std::io::Write;
                        use conversion::varint::to;
                        // Packet id is One byte
                        let mut packet_size = 1; 
                        $(
                                packet_size += ($data).get_size();
                        )*
                        let _ = $stream.write( &(to(&mut (packet_size)))[..]);
                        let _ = $stream.write( &[($packet_id as u8)] );
                        let mut piece = 0;
                        $(
                                let _ = $stream.write($data.convert());
                        )*
                        let _ =$stream.flush();
                }
        };
}
use std::mem;
use conversion::varint;
use std::any::Any;
use std::slice::from_raw_parts_mut;
pub trait CanSend {
        fn get_size(&self) -> i32;
        fn convert(&mut self) -> &[u8];
}

impl<T: Any + 'static> CanSend for T {
        fn get_size(&self) -> i32 {
                let from = self as &Any;
                if let Some(string) = from.downcast_ref::<String>() {
                        (string.len() + varint::to(&mut ((string.len()) as i32)).len()) as i32
                } else if let Some(vector) = from.downcast_ref::<Vec<u8>>() {
                        vector.len() as i32
                } else {
                        mem::size_of::<T>() as i32
                }
        }
        fn convert(&mut self) -> &[u8] {
                let mut from = self as &mut Any;
                if from.is::<String>() {
                        let string = from.downcast_mut::<String>().unwrap();
                        let string_as_bytes = string.clone().into_bytes();
                        *string  = unsafe {
                                String::from_utf8_unchecked(
                                        varint::to(&mut (string_as_bytes.len() as i32))
                                        .iter()
                                        .chain(string_as_bytes.iter())
                                        .map( |x| *x)
                                        .collect::<Vec<u8>>()
                                )
                        };
			string.as_bytes()
                } else if from.is::<Vec<u8>>() {
                        let vector = from.downcast_mut::<Vec<u8>>().unwrap();
                        &vector[..]
                } else  {
                        //reverses the order of bytes,
                        unsafe {
                                let raw:*mut u8 = mem::transmute(from.downcast_mut::<T>().unwrap() );
                                let size = mem::size_of::<T>();
                                let end = size -1;
                                let t_as_u8_slice = from_raw_parts_mut(raw, size);
                                for i in 0..(size/2) {
                                        t_as_u8_slice[i] |= t_as_u8_slice[end -i];
                                        t_as_u8_slice[end - i] |= t_as_u8_slice[i];
                                        t_as_u8_slice[i] |= t_as_u8_slice[end -i];
                                }
                                t_as_u8_slice
                        }
                }	
        }
}


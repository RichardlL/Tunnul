/*
    Tunul - Minecraft compatible server
    Copyright (C) 2015 Richard Lettich

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::u32;
err
fn main() {
        let p = [from_varint];
        unsafe{ let x:[u8; 4] = std::mem::transmute(-1);
        let (a, b) = p[0](&x); 
        println!("{}",a);}
}

// Takes client packets, and decides what to do with them
// We cant use STD from big edian since its a varint
// Confusing? It was confusing to write
// You would think this needs error handling, but it doesnt
// Funny how that works.
fn from_varint(src_array: &[u8])-> (i32, usize) {
        let mut result:i32 = 0;
        let mut vi_size:usize = 0;
        let pl = src_array.len();
        while vi_size < pl {
                //result |=   p's first free digits  shifted all the way
                result |= (src_array[vi_size] as i32 & 0x7f )  << (25 - (7 * vi_size)) ;  //25 - 7v is equal to 32 - 7 * (vi_size -1)
                // if first bit is 0
                if src_array[vi_size] & 0x80 == 0 {
                        break;
                }
                vi_size += 1;
        }
        // total number of 7 bytes read
        // We dont need to account for twos complement (see wikipedia) , i32 bitshift already does
        result = result >> (39 - (7 * vi_size ));  // equivlent to 32 - (vi_size - 1) 
        (result, vi_size)
}

//
fn to_varint<T>(src_array: &[u8])-> (i64, usize) {
        let mut result:i64 = 0;
        let mut vi_size:usize = 0;
        let pl = src_array.len();
        while vi_size < pl {
                //result |=   p's first free digits  shifted all the way
                result |= (src_array[vi_size] as i64 & 0x7f )  << (57 - (7 * vi_size)) ;  //25 - 7v is equal to 32 - 7 * (vi_size -1)
                // if first bit is 0
                if src_array[vi_size] & 0x80 == 0 {
                        break;
                }
                vi_size += 1;
        }
        // total number of 7 bytes read
        // We dont need to account for twos complement (see wikipedia) , i32 bitshift already does
        result = result >> (71 - (7 * vi_size ));  // equivlent to 32 - (vi_size - 1) 
        (result, vi_size)
}
/*
fn handle_packet(packet: &[u8]) -> () {
        let (packet_size, vi_size)= from_varint(packet);
        if  packet_size + vi_size as i32 != (*packet).len() as i32 {
                println!("Error, packetsize wrong");
        }
        let (packet_id, vi_size) = from_varint(&packet[vi_size+1..]);
        let p = [from_varint];
        let x:[u8; 4] = unsafe { std::mem::transmute(1)};
        p[packet_id as usize](&x);
}

*/












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

fn main() {
    let x:[u8; 4] = udsansafe { std::mem::transmute(1)};

}

//takes client packets, and decides what to do with them
// we cant use STD from big edian since its a varint
fn from_varint(packet: &[u8])-> (u32, usize) {
        let mut result:u32 = 0;
        let mut vi_size:usize = 1;
        for p in packet {
                result |= (*p as u32 & 0x7f )  << ((8- vi_size) * 8 + vi_size) ;
                if p  & 0x80 == 0 {
                        break;
                }
                vi_size += 1;
        }
        result >> (32 - (7 * vi_size));
        (result, vi_size)
}

fn handle_packet(packet: &[u8]) -> () {
        let (packet_size, vi_size)= from_varint(packet);
        if  packet_size + vi_size as u32 != (*packet).len() as u32 {
                println!("Error, packetsize wrong");
        }
        let (packet_id, vi_size) = from_varint(&packet[vi_size+1..]);
}














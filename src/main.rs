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

extern crate num;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::Read;

fn main() {
        println!("Starting Tunul Server...");
        let connections = match TcpListener::bind("127.0.0.1:25565"){
                Ok(c) => c,
                Err(_) => panic!("Error Listening: Is a server running?"),
        };
        println!("Bound Server Successfully, Open for Connections");
        for stream in connections.incoming() {
                thread::spawn(move|| {packet_reader(stream.unwrap())});
        }
}

struct LoginPacket {
        id: usize,
        data: Vec<u8>,
}
impl LoginPacket {
        fn new(stream: &mut TcpStream) -> LoginPacket {
                let (length, _) = read_varint(stream.by_ref());
                let (packetid , so_packetid) = read_varint(stream.by_ref());
                let mut buff: Vec<u8> = Vec::with_capacity(length as usize - so_packetid);
                let _ = stream.read(&mut buff);
                LoginPacket {id: packetid as usize, data: buff}
        }
       /* fn handle(&self) -> Option<bool> {
                
        }*/
}

fn packet_reader(stream: TcpStream) {
        println!("Player connected, Player ip: {}",stream.peer_addr().unwrap());
        let mut stream = stream;
        let _ = stream.set_read_timeout(Some(Duration::new(30, 0)));
        let _ = stream.set_read_timeout(None);
        let p = LoginPacket::new(stream.by_ref());
        println!("packet id: {}", p.id);
        let _ = p.data; /* FIXME: */
}

fn read_varint(src_array:&mut TcpStream) -> (i64, usize) {
        let mut result:i64 = 0;
        let mut vi_size:usize = 0;
        let mut temp: u8;
        loop {
                temp = src_array.bytes().next().unwrap().unwrap();
                result |= ((temp & 0xFF ) as i64)  << (57 - (7 * vi_size)) ;
                if temp & 0x100u8 == 0 {
                        break
                }
                vi_size += 1;
        }
        vi_size += 1;
        (result >> (32 - (7 * (vi_size) )), vi_size)
}

fn to_varint(src: i64) -> Box<[u8]> {
        let src_size = std::mem::size_of::<i64>() * 8;
        let plus_num = |x, y| -> Box<[u8]> {
                let bytes:usize = ((src_size - (src.leading_zeros() as usize)) * 9  + 7) / 8;
                let mut result: Vec<u8> = vec![0; bytes];;
                for i in 0..bytes+1 {
                        result[10-i] = (y ^ (x << i)) as u8;
                }
                result[10] &= 0xFF;
                result.into_boxed_slice()
        };
        let firstdigit = 0x1 << 63i64;
        match src  & firstdigit {
                0x100000000  => plus_num(src ^( 0x100000000i64 >> 63),0x1FF),
                _  => plus_num(src,0x100),
        }
}



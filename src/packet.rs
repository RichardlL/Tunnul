/* Tunnul - Minecraft server1
 * Copyright 2015 Richard Lettich
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
 * IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
 */

// Note: All packets sent after login will be in player.rs

// Packet id is defined by minecraft so you know how to handle the
// packet e.g. block updates, movement, etc.
// Data is just the rest of the packet, and will vary based on id
pub struct Packet {
        pub id: usize,
        pub data: Vec<u8>,
        pub index: usize,
}

// rather than just a vector, well also store a index, so we dont have
// to keep track of what we have already read

use std::time::Duration;
use std::net::TcpStream;
use conversion;
use std::io::Read;
use std::{str,string};
use std::mem;

impl Packet {
        //Takes a tcp stream and pulls a packet from it
        //MAJOR FIX : no guarantee of full packet
        //MAJOR FIX: prevent over allocation
        pub fn new(stream: &mut TcpStream) -> Packet {
                let mut stream = stream;
                let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));

                let (length, _) = conversion::itt::read(stream);
                let (packetid , sizeof_packetid) = conversion::itt::read(stream);
                let length_to_read = (length as usize) - sizeof_packetid;
                let mut buff:Vec<u8> = vec![0;length_to_read];
                let _ =stream.read(&mut buff);
                Packet { id: packetid as usize, data: buff,  index: 0 }
        }
        // Gets varint from current index position and updates index
        pub fn get_varint(&mut self) -> i64 {
        	let mut result:i64 = 0;
        	let mut vi_size:usize = 0;
        	loop {
                	result |= ((self.data[vi_size] & 0x7Fu8 ) as i64)  << (7 * vi_size) ;
                	if self.data[vi_size] & 0x80u8 == 0 {
                        	break;
                	}
                	vi_size += 1;
        	}
        	vi_size += 1;
        	self.index += vi_size;
        	result |= ((result & 0x40) << 57) >> (63 - (7 * vi_size));
        	result
	}
        //gets string from current index and updates position
        pub fn get_str(&mut self) -> &str {
                let end = self.get_varint()as usize + 1;
                let begin = self.index;
                self.index = end;
                str::from_utf8(&self.data[begin..end]).unwrap()
        }
        pub fn get_string(&mut self) -> String {
                string::ToString::to_string(self.get_str())
        }
        // Gets Type T and updates buffer, where T is Statically sized
	pub fn get<T>(&mut self) -> T {
	        let start = self.index;
	        self.index += mem::size_of::<T>();
	        unsafe { mem::transmute_copy(&[start, self.index].reverse())} 
	}
}


// Checks if client wants the server's status or to join
// player_connect goes to player/mod.rs

// Were spawning a new thread rather than using this one
// to release the stream (streams are copies),
// and to release the first packet
use std::thread;
use player;
pub fn new_connection(stream: TcpStream) {
        let mut stream = stream;
        let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));
	let new_player_packet = Packet::new(&mut stream);
        match new_player_packet {
                //Packet { id: 0 , data: d, index:_} if d.is_empty() => {}, //FEATURE new_player_packet.ping_response(),
                Packet { id:0, ..} =>  thread::spawn(move|| {player::player_login(new_player_packet, stream)}),
                Packet{..} => panic!("Malformed login packet"),
        };
}
pub fn send_status(stream: TcpStream) {
        unimplemented!();
}
use std::io::Write;
pub fn wrong_version(mut stream :TcpStream, client: u8, server: u8) {
        let mut temp = format!("{{\"text\": \"Version of Minecraft Not Compatible, \n Your Protocol Version is: {} \n Server Verrsion: {}}}", client, server);
        Send! { &mut stream,
                0x0,
                temp
        };
}

// checks for new players
// If write fails (stream closed), it stops sending keep alive
use std::sync::mpsc;
pub fn keep_alive_loop(rx: mpsc::Receiver<TcpStream>) {
        let mut connections:Vec<TcpStream> = Vec::new();
        loop {
                match rx.try_recv() {
                        Ok(stream) => connections.push(stream),
                        Err(_) => (),
                }
                let mut length = connections.len();
                let mut i:usize = 0;
                while i < length {
                        // Having it be random is useless
                        if connections[i].write(&[0x2u8,0x0u8,0x0u8]).is_err() {
                                connections.swap_remove(i);
                                length -= 1;
                                println!("Client Disconnected");
                                continue;
                        }
                        i += 1;
                }
                thread::sleep(Duration::new(5, 0));
        }
}

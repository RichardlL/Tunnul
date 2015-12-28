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


// Note: All packets sent after login will be in player.rs

// Packet id is defined by minecraft so you know how to handle the
// packet e.g. block updates, movement, etc.
// Data is just the rest of the packet, and will vary based on id
pub struct Packet {
    pub id: u8,
    pub data: Vec<u8>,
    pub index: usize,
}

// rather than just a vector, well also store a index, so we dont have
// to keep track of what we have already read

use std::time::Duration;
use std::net::TcpStream;
use conversion::itt;
use std::io::Read;
use std::str;
use std::mem;
use std::slice::from_raw_parts_mut;
impl Packet {
    // Takes a tcp stream and pulls a packet from it
    // MAJOR FIX : no guarantee of full packet
    // MAJOR FIX: prevent over allocation
    pub fn new(mut stream: &mut TcpStream) -> Result<Packet, &'static str> {
        let _ = stream.set_read_timeout(Some(Duration::from_secs(20)));
        let data_l =  itt::read(stream) - 1;
        if data_l > 1024 || data_l < 0 {
            return Err("");
        }
        let mut packet = Packet {
            id: 0,
            data: vec![0; data_l as usize],
            index: 0,
        };
        packet.id = itt::read(stream) as u8;
        let _ = stream.read_exact(&mut packet.data);
       // println!("Packet id: {}", packet.id);
        Ok(packet)
    }
    // Gets varint from current index position and updates index
    pub fn get_varint(&mut self) -> i64 {
        let mut result: i64 = 0;
        let mut vi_size: usize = 0;
        loop {
            result |= ((self.data[vi_size] & 0x7Fu8) as i64) << (7 * vi_size);
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
    // gets string from current index and updates position
    pub fn get_str(&mut self) -> Result<&str, &'static str> {
        let begin = self.index;
        let end = begin + self.get_varint() as usize + 1;
        if end - begin > 100 {
            return Err("Sent String to large :\\ ");
        }
        self.index = end;
        Ok(str::from_utf8(&self.data[begin..end]).unwrap())
    }
    pub fn get_string(&mut self) -> Result<String, &'static str> {
        Ok(try!(self.get_str()).to_owned())
    }
    // Gets Type T and updates buffer, where T is Statically sized
    pub fn get<T: Clone>(&mut self) -> T {
        let start = self.index;
        self.index += mem::size_of::<T>();
        unsafe {
            let result:T = mem::uninitialized();
            let r_slice = from_raw_parts_mut(mem::transmute::<_,*mut u8>(&result), mem::size_of::<T>());
            for (i, byte) in self.data[start..self.index].iter().rev().enumerate() {
                r_slice[i] = *byte;
            }
            result
        }
    }
}
use player::ReceiverData;
pub fn form_packet(mut stream: TcpStream, tx: Sender<ReceiverData>)  {
    loop {
        match Packet::new(&mut stream) {
            Ok(p) => {
                if tx.send(ReceiverData::Packet(p)).is_err() {
                     return; 
                }
            },
            Err(e) => {
                let _ = tx.send(ReceiverData::Err);
                return;
            },
        }
    }
}
// Checks if client wants the server's status or to join
// player_connect goes to player/mod.rs

// Were spawning a new thread rather than using this one
// to release the stream (streams are copies),
// and to release the first packet
use std::thread;
use player;
use player_loop::player_loop;
pub fn new_connection(stream: Box<TcpStream>, tx: Sender<Sender<ReceiverData>>) {
    let mut stream = stream;
    let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));
    let mut new_player_packet = Packet::new(&mut stream).unwrap();
    match new_player_packet {
        // Packet { id: 0 , data: d, index:_} if d.is_empty() => {},
        Packet { id:0, ..} => (),
        Packet{..} => { println!("Malformed login packet"); return},
    };
    // We are handling everything manually here
    // SETTING: Version number (of minecraft packet protocol)
    let vers = 47u16;
    let client_vers = new_player_packet.get_varint() as u16;
    // Minecraft gives the server its ip adress(prefixed with varint), not needed for now
    new_player_packet.index += new_player_packet.get_varint() as usize + 2;
    // check If they just want to ping (1 is ping , two is login)
    // protocol is just to send empty packet, so we dont need to read it :0
    if 1 == new_player_packet.get_varint() {
        let _ = Packet::new(&mut stream);
        //send_status(&mut stream);
    } else if vers != client_vers {
        wrong_version(&mut stream, client_vers as u8, vers as u8);
    } else {
        let player = player::Player::from_stream(stream);
        tx.send(player.tx.clone());
        player_loop(player);
    }
}
//pub fn send_status(stream: &mut TcpStream) {
 //   unimplemented!();
//}
use std::io::Write;
pub fn wrong_version(mut stream: &mut TcpStream, client: u8, server: u8) {
    let mut temp = format!("{{\"text\": \"Version of Minecraft Not Compatible, \n Your Protocol \
                            Version is: {} \n Server Verrsion: {}}}",
                           client,
                           server);
    Send!{ &mut stream,
                0x0,
                temp
        };
}

// checks for new players
// If write fails (stream closed), it stops sending keep alive
use std::sync::mpsc::{Sender, Receiver};
pub fn keep_alive_loop(rx: Receiver<Sender<ReceiverData>>) {
    let mut connections: Vec<Sender<ReceiverData>> = Vec::new();
    loop {
        if let Ok(keep_alive_tx) = rx.try_recv() {
            connections.push(keep_alive_tx);
        }
        let mut p = connections.len();
        while p != 0 {
            p -= 1;
            if connections[p].send(ReceiverData::KeepAlive).is_err() {
                connections.swap_remove(p);
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}

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

// This module Handles initial player connections, and associated responses

use std::thread;
use player;
use player::Player;
use player_loop::player_loop;
use packet::Packet;
use std::time::Duration;
use std::sync::mpsc::Sender;
use player_loop::ReceiverData;

pub fn new_connection(mut stream: Box<TcpStream>, keep_alive_tx: Sender<Sender<ReceiverData>>) {
    const PROTOCOL_VERSION: u8 = 47;

    let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));

    let mut pack = Packet::new(&mut stream).unwrap();
    let client_vers = pack.get_varint() as u8;
    let is_ping = pack.get_varint();

    match (client_vers, is_ping) {
        (_, 1) => (),
        (PROTOCOL_VERSION, _) => {
            let mut player = match player::Player::from_stream(stream) {
                Some(p) => p,
                None => return,
            };
            keep_alive_tx.send(player.tx.clone());
            
            player.confirm_login();
            player.join_game();
            player.send_spawn();
            player.send_location();
            
            thread::spawn(move || player_loop(player));  // Spawning again to drop stack.
        }, // Sendstatus
        (_, 0) => wrong_version(&mut *stream, client_vers, PROTOCOL_VERSION),
        _ => println!("Malformed login packet {}, {},",client_vers,is_ping),
    }
}

impl Player {
    fn confirm_login(&mut self) {
        Send!{
            &mut self.stream,
            0x2u8 ,
            "de305d54-75b4-431b-adb2-eb6b9e546014".to_string(),
            self.name
        };
    }
    fn join_game(&mut self) {
        Send!{ 
            &mut self.stream,
            0x1,
            self.eid,
            self.game_mode,
            self.world_type,
            0x0u8,  // Fixme, Difficulty
            0b11111111u8, //max players
            "default".to_string(),
            0x0u8
        }
    }
}

use std::io::Write;
use std::net::TcpStream;
fn wrong_version(mut stream: &mut TcpStream, client: u8, server: u8) {
    let mut temp = format!("{{\"text\": \"Version of Minecraft Not Compatible, \n Your Protocol \
                            Version is: {} \n Server Verrsion: {}}}",
                            client,
                           server);
    Send!{ 
        &mut stream,
        0x0,
        temp
    };
}


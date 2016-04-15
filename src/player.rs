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


// -/////////
// Player
// /////-*/

/// EID:
/// Either hash of ip and username, so player doesnt have to login
/// OR
/// hash of username and password (optional, but allows users to use multiple ip's) (Unimplemented)
/// Used to identify player, and so multiple people can use the same name
/// Since their is no fair way of managing who gets what name, we let everyone have any name they
/// choose, This way no one can hog names, or we dont have to limit names per ip adress
/// The Color of username will be set to first 12 bytes of this (hex color), and prepened with
/// The first 3 bytes

// Action: (bool, Option<Coordinate>),
// if a player is currently digging a block. To keep track of
// how keep from digging to fast, or performing multiple actions
// (transfering stuff from chest, while running/mining)
// the bool is true if using, not mining (chest, craftbench , etc. can be both)

///  xmin, xmax, zmin, zmax
/// The corners of the chunk the player is in; we Have to keep track, as players dont unload chunks automatically

// Respawn: 
// players home, sethome
// **NOT SPAWN**

// Window:
// Window's (any inventory except the players e.g. chest)
// inventory data

// World type:
// -1..1 {nether, regular,  end}

// game_type
// 0..2 {survival, creative, adventure}
use std::sync::mpsc::{Sender, Receiver, channel};
/// Either a Packet from Client, or update from another thread (ie entity moved)
use packet::Packet;
use struct_types::Location;
use player_loop::ReceiverData;

use std::io::Write;
use std::net::TcpStream;
use to_client;
pub struct Player {
    pub eid: u32,
    pub name: String,
    pub location: Location,
    pub pitch: f32,
    pub yaw: f32,
    // pub interation: (Option<Location>, PreciseTime),
    pub chunks: Vec<(i32, i32)>,
    pub last_on_ground: Location, //Includes last "on ground, to calculate fall damage
    pub health: i16,
    pub food: i8,
    pub food_saturation: f32,
    pub world_type: i8,
    pub game_mode: u8,
    pub respawn: Location,
    pub reputation: u8,
    pub tx: Sender<ReceiverData>,
    pub rx: Receiver<ReceiverData>,
    pub stream: Box<TcpStream>,
}
use std::hash::{Hash, SipHasher, Hasher};
use std::thread;
use packet;

impl Player {
    // Logins in player if existing found, or creates new
    // Feature: record and check
    pub fn from_stream(mut stream: Box<TcpStream>) -> Option<Player> {
        let mut login_packet = match Packet::new(&mut stream) {
            Ok(p) => p,
            _ => {
                println!("Error Logging player In");
                return None;
            },
        };
        let player_name = login_packet.get_string();

        let mut hash = SipHasher::new();
        stream.peer_addr().unwrap().ip().hash(&mut hash);
        hash.write(player_name.as_bytes());

        // Sends `RecieveverData`, can be either from tcp in, or other threads
        // We need to notify player thread, incase Something needs to be updated.
        let (to_player,  data_rx) = channel();
        
        let to_player_clone = to_player.clone();
        //let stream_clone = stream.try_clone().unwrap();
        //thread::spawn(move || packet::form_packet(stream_clone, to_player_clone));

        Some(Player {
            eid: hash.finish() as u32,
            name: player_name,
            chunks: Vec::new(),
            location: Location::new(),
            yaw: 0.0,
            pitch: 0.0,
            respawn: Location::new(),
            last_on_ground: Location::new(),
            health: 20,
            food: 20,
            food_saturation: 5.0,
            world_type: 0,
            game_mode: 1,
            reputation: 0,
            tx: to_player,
            rx: data_rx,
            stream: stream
        })
    }
}


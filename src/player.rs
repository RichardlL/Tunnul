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

/*-/////////
Player
////////-*/

// EID:
// Either hash of ip and username, so player doesnt have to login
// OR
// hash of username and password (optional, but allows users to use multiple ip's)

// Name Prefix:
// Since their is no way of managing who gets what name, we let everyone have any name they choose
// This way no one can hog names, or we dont have to limit names per ip adress
// This will be bitwise XOR of hash in case of ip login, or we'll let
// users who register choose
// This will prevent scamming

// action: (bool, Option<Coordinate>),
// if a player is currently digging a block. To keep track of
// how keep from digging to fast, or performing multiple actions (transfering stuff from chest, while running/mining)
// the bool is true if using, not mining (chest, craftbench , etc. can be both)

// Chunck: 
// the player is in; we Have to keep track, as players dont unload chunks automatically

// food saturation:
// some foods (meat) will fill this up, if food is full, 'hunger" take away from saturation
// rather than food

// Respawn:
// players home, sethome 
//**NOT SPAWN**

// Window:
// Window's (any inventory except the players e.g. chest)
// inventory data

// World type:
// -1..1 {nether, regular,  end}

//game_type
// 0..2 {survival, creative, adventure}
#[macro_use]
/*fn exact<T: Sized>(input: T) -> Box<u8> {
        
}*/
use std::mem;
use packet_sending;
use packet_sending::CanSend;
pub struct Location {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub pitch: f32,
        pub yaw: f32,
}
impl Location {
        fn new() -> Location {
                Location {
                        x: 10.0,
                        y: 60.0,
                        z: 10.0,
                        pitch: 0.0,
                        yaw: 0.0
                }
        }
        fn form_postition(&self) -> u64 {
	let result = (((self.x as u64) & 0x3FFFFFFu64) << 38)
                        | ((self.z as u64) & 0x3FFFFFFu64)
                        | (((self.y as u64)& 0xFFFu64) << 26);
	result.swap_bytes() // This is should remain little edian.
        }
        // Note - the actual distance is the square root of this, this is simply for hacking, etc
        pub fn distance(&self, loc: &Location) -> f64 {
                (self.x - loc.x).powi(2) + (self.z - loc.z).powi(2)
        }
}

use std::io::Write;
use std::net::TcpStream;

pub struct Player {
        eid: u32,
        pub name: String,
        pub location: Location,
        on_ground: bool,
        health: f32,
        food: f32,
        food_saturation: f32,
        world_type: i8,
        game_mode: u8,
        respawn: Location,
        reputation: u8,
        stream: TcpStream
}
use std::hash::{Hash, SipHasher, Hasher};


impl Player {
        // Logins in player if existing found, or creates new
        // Feature: record and check
        fn from_stream(stream: &mut TcpStream) -> Player {
                let mut login_packet = Packet::new(&mut stream.try_clone().unwrap());
                let player_name = login_packet.get_string();
                let mut hash_gen = SipHasher::new();
                stream.peer_addr().unwrap().ip().hash(&mut hash_gen);
                hash_gen.write(player_name.as_bytes());
                let hash = hash_gen.finish();
                Player {
                        eid: ((hash & 0xFFFFFFFF) as u32),
                        name: player_name,
                        location: Location::new(),
                        respawn: Location::new(), //user server spawn
                        on_ground: true,
                        health: 20.0,
                        food: 20.0,
                        food_saturation: 5.0,
                        world_type: 0,
                        game_mode: 1,
                        reputation: 0,
                        stream: stream.try_clone().unwrap()
                }
        }
        fn confirm_login(&mut self) {
                Send! {&mut self.stream, 0x2u8 , "de305d54-75b4-431b-adb2-eb6b9e546014".to_string(), self.name.clone() };
        }
        fn join_game(&mut self) {
                Send! { &mut self.stream,
                        0x1,
                        self.eid.clone(),
                        self.game_mode.clone(),
                        self.world_type.clone(),
                        0x0u8,  // Fixme, Difficulty
                        0b11111111u8, //max players
                        "default".to_string(),
                        0x0u8
                }
        }
        fn send_spawn(&mut self) {
                Send! { &mut self.stream, 0x5u8, self.respawn.form_postition().to_be() };
        }
        fn send_location(&mut self) {
                Send! { &mut self.stream,
                        0x8u8,
                        self.location.x.clone(),
                        self.location.y.clone(),
                        self.location.z.clone(),
                        self.location.pitch.clone(),
                        self.location.yaw.clone(),
                        0x0u8 
                };
        }
}

use packet;
use packet::{Packet};
use conversion;

pub fn player_login(mut first_packet: Packet, mut stream: TcpStream) {
        // We are handling everything manually here
        //SETTING: Version number (of minecraft packet protocol)
        // version comes first in packet, but we dont need that if they just want to ping us,
        // so well save it for later
        let vers = 47u8;
        let client_vers = first_packet.get_varint() as u8;
        //Minecraft gives the server its ip adress(prefixed with varint), not needed for now
        first_packet.index += first_packet.get_varint() as usize + 2;

        //check If they just want to ping (1 is ping , two is login)
        //protocol is just to send empty packet, so we dont need to read it :0
        if 1 == first_packet.get_varint() {
                let _ =Packet::new(&mut stream);
                packet::send_status(stream); 
        } else if vers != client_vers {
                packet::wrong_version(stream, client_vers, vers);
        } else {
                let mut player = Player::from_stream(&mut stream);
                player.confirm_login();
                player.join_game();
                player.send_spawn();
                player.send_location();
                println!("{}, has Joined this dank server", &player.name);
        }
}




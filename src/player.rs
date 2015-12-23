// Tunnul - Minecraft server
// Copyright 2015 Richard Lettich
//
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
// IT IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
//

// -/////////
// Player
// /////-*/

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
// how keep from digging to fast, or performing multiple actions
// (transfering stuff from chest, while running/mining)
// the bool is true if using, not mining (chest, craftbench , etc. can be both)

// Chunck:
// the player is in; we Have to keep track, as players dont unload chunks automatically

// food saturation:
// some foods (meat) will fill this up, if food is full, 'hunger" take away from saturation
// rather than food

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
            x: 15.0,
            y: 60.0,
            z: 88.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
    fn form_postition(&self) -> u64 {
        // position is same a f64, but tells send macro not to switch bytes
        (((self.x as u64) & 0x3FFFFFFu64) << 38) | ((self.z as u64) & 0x3FFFFFFu64) |
        (((self.y as u64) & 0xFFFu64) << 26)
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
    stream: Box<TcpStream>,
}
use std::hash::{Hash, SipHasher, Hasher};

impl Player {
    // Logins in player if existing found, or creates new
    // Feature: record and check
    pub fn from_stream(mut stream: Box<TcpStream>) -> Player {
        let mut login_packet = Packet::new(&mut stream);
        let player_name = login_packet.unwrap().get_string().unwrap();
        let mut hash_gen = SipHasher::new();
        stream.peer_addr().unwrap().ip().hash(&mut hash_gen);
        hash_gen.write(player_name.as_bytes());
        let hash = hash_gen.finish();
        Player {
            eid: ((hash & 0xFFFFFFFF) as u32),
            name: player_name,
            location: Location::new(),
            respawn: Location::new(), // user server spawn
            on_ground: true,
            health: 20.0,
            food: 20.0,
            food_saturation: 5.0,
            world_type: 0,
            game_mode: 1,
            reputation: 0,
            stream: stream,
        }
    }
    pub fn confirm_login(&mut self) {
        Send!{&mut self.stream,
            0x2u8 ,
            "de305d54-75b4-431b-adb2-eb6b9e546014".to_string(),
            self.name
        };
    }
    pub fn join_game(&mut self) {
        Send!{ &mut self.stream,
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
    pub fn send_spawn(&mut self) {
        Send!{ &mut self.stream, 0x5u8, self.respawn.form_postition() };
    }
    pub fn send_location(&mut self) {
        Send!{ &mut self.stream,
                        0x8u8,
                        self.location.x,
                        self.location.y,
                        self.location.z,
                        self.location.pitch,
                        self.location.yaw,
                        0x0u8
                };
    }
}
use packet;
use packet::Packet;
use player_loop::player_loop;
pub fn player_login(mut first_packet: Packet, mut stream: TcpStream) {

}

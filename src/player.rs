/*
 * Tunul - Minecraft server
 * Copyright 2015 Richard Lettich
 *
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT.
 * TUNUL IS NOT APPROVED BY OR ASSOCIATED WITH MOJANG.
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

use std::mem;

pub struct Location {
        x: f64,
        y: f64,
        z: f64,
        pitch: f32,
        yaw: f32,
}
impl Location {
        fn new() -> Location {
                Location {
                        x: 0.0,
                        y: 60.0,
                        z: 0.0,
                        pitch: 0.0,
                        yaw: 0.0
                }
        }
        fn form_postition(&self) -> u64 {
                let result:u64 = (((self.x as u64) & 0x3FFFFFFu64) << 38)
                        | ((self.z as u64) & 0x3FFFFFFu64)
                        | (((self.y as u64)& 0xFFFu64) << 26);
                result
        }
}

pub struct Player {
        eid: u32,
        name: String,
        location: Location,
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
        fn confirm_login(&self) {
                // uuid is for Minecraft Server login, different than the  EID hash we used
                // Since I have decided to not use mojang to autheticate, this is garbage filler to make it compatibale
                let name= &conversion::to_string(self.name.clone());
                let uuid = &conversion::to_string("de305d54-75b4-431b-adb2-eb6b9e546014".to_string());
                packet::form_packet(self.stream.try_clone().unwrap(), 0x02, &[&uuid[0], &uuid[1] , &name[0], &name[1]]);
        }
        fn join_game(&self) {
                let eid:[u8;4] = unsafe { mem::transmute_copy(&self.eid)};
                let gamemode:u8 = self.game_mode.clone();
                let world_type:u8 = self.world_type.clone() as u8;
                let difficulty = 0x0u8; //Feature: difficulty
                let max_players = 0x1111111u8;
                let leveltype = conversion::to_string("default".to_string());
                let debug_allow = 0x0u8; //only applies to vanilla players, so useless to use...
                packet::form_packet_bytes(&mut self.stream.try_clone().unwrap(), 0x1u8, &[
                        &eid[..],
                        &[gamemode,
                        world_type,
                        difficulty,
                        max_players],
                        leveltype[0].as_ref(),
                        leveltype[1].as_ref(),
                        &[debug_allow]]);
        }
        fn send_spawn(&mut self) {
                let data:[u8;8] = unsafe{ mem::transmute(self.respawn.form_postition())};

                packet::form_packet_bytes(&mut self.stream, 0x5u8, &[&data])
        }
        fn send_location(&mut self) {
                let data:[u8;24] = unsafe { mem::transmute([
                        self.location.x,
                        self.location.y,
                        self.location.z,])};
                let data2:[u8;8] = unsafe { mem::transmute([
                        self.location.pitch,
                        self.location.yaw])};
                packet::form_packet_bytes(&mut self.stream,0x8u8,&[&data,&data2,&[0x0u8]]);
        }
}

use packet;
use packet::{Packet};
use std::net::TcpStream;
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
        if 1 == first_packet.get_varint() {
                //protocol is just to send empty packet, so we dont need to read it :0
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




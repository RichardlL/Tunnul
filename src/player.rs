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

pub struct Player {
       // location: Coordinate,
        on_ground: bool,         // If the player is on the ground or in air (flying, falling, etc)
        yaw: f32,
       // action: (bool, Option<Coordinate>),
                                 // if a player is currently digging a block. To keep track of
                                 // how fast they are digging, and prevent mining multipe blocks
                                 // the bool is if its utilizing a window rather than mining (chest, craftbench)

        chunks: (i32,i32),       // chunck the player is in; we Have to keep track, as players dont unload chunks automatically
        //inventory: Inventory	 //
        health: f32,             //
        food: f32,               // 0..20
        food_saturation: f32,     // see wiki
        worldType: i8,           // -1..1 {nether, regular,  end}
        game_mode: u8,           // 0..2 {survival, creative, adventure}
      //  spawn_point: Coordinate, // respawn
        //window: window           // if player is in craftbench, chest, etc
        //abilities                // speed, supermine, etc
        reputation: u8,          // Keep track of violations, tolerance for move to fast, etc
}
use std::thread;
use packet::{Packet};
use std::net::TcpStream;
pub fn player_connect(mut first_packet: Packet, mut stream: TcpStream) {
        // Protocol version check
        let p_v = first_packet.get_varint();
        if p_v != 47 {
                panic!("ERROR: player Disconnected, incorrect version");
        }
        // Minecraft sends the server adress and port;
        // this is used for authentication, but we dont have that right now
        first_packet.index += first_packet.get_varint() as usize + 2;
        // This is if they want server status or to login
        if first_packet.get_varint() == 1 {
                // status update 
        } else {
                let mut login_packet = Packet::new(&mut stream);
                let player_name = login_packet.get_string();
                println!("{} has joined",player_name);
        }
        loop {
                thread::sleep_ms(2000);
        }
}








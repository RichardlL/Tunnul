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
       
        chunks: (i32,i32),       // chunck the player is in;we Have to keep track, as players dont unload chunks automatically
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

use packet::{Packet};
use std::net::TcpStream;
pub fn player_connect(first_packet: Packet, stream: TcpStream) {
        // Protocol version check. This implements 47
        if let 47 = first_packet.get_varint() {
                return;
        }
}








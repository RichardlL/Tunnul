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
use std::{thread,time};
use packet;
use packet::{Packet};
use std::net::TcpStream;
use conversion;

pub fn player_connect(mut first_packet: Packet, mut stream: TcpStream) {
        //SETTING: Version number (of minecraft packet protocol)
        //version comes first, but we dont need that if they just want to ping us,
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
                let mut login_packet = Packet::new(&mut stream);
                let player_name = login_packet.get_string();
                /*
                *FEATURE/FIXME : Autheticate login, or be able to set a password in game
                */
                //sends Login Success
                confirm_login(&mut stream, player_name);
        }
}
use std::borrow;
fn confirm_login(mut stream: &TcpStream,  name: borrow::Cow<str>) {
        println!("{} has joined the game len {}", name, name.len());
        let length = conversion::varint::to(((*name).len() as i32 - 1));
        println!("leeeee {}", length[0]);
        packet::form_packet(stream, &[&[13], name.as_bytes()],0x02);
}




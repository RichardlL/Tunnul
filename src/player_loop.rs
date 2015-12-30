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


// This file contains the player loop and all functions for handling received packets
use player;
use packet;
use std::net::TcpStream;
use std::time::Duration;
use std::io::Write;

// Easier to optimize than more intuitive solutions
fn packet_handler(player: &mut Player, pack: &mut Packet) -> Option<&'static str> {
    match pack.id {
        0 => None,
        1 => chat_message(player, pack),
        2 => use_entity(player, pack),
        3 => is_flying(),
        4 => position_update(player, pack),
       // 5 => look_update(player, pack),
        //6 => position_and_look_update(player, pack),
       // 7 => player_digging(player, pack),
       // 8 => block_placement(player, pack),
        //9 => held_item_update(player, pack),
        //10 => action(player, pack),
        //11 => vehichle_steer(player, pack),
        //12 => close_window(player, pack),
        //13 => click_inventory_slot(player, pack),
        //14 => confirm_transaction(player, pack),
        //15 => creative_inventory(player, pack),
        //16 => enchant_item(player, pack),
        //17 => sign_set(player, pack),
        //18 => player_abilities(player, pack),
        //19 => tab_complete(player, pack),
        //20 => client_settings(player, pack),
        //21 => spawn_request(player, pack),
        //22 => plugin_message(player, pack),
        //23 => spectate(player, pack),
        //24 => resource_pack_status(player, pack),
        _ => None, //panic!("invalid packet id"),
    }
}
// Types of Data tx's send that can unblock player thread
pub enum ReceiverData {
    Packet(Packet),
    TcpErr, // Packet already formed
    KeepAlive,
    // Fixme, Thread to thread transmission
}
pub fn player_loop(mut player: Player) {
    println!("{} has joined the game", player.name);
    loop {
        // Large match Easier to optimize than more intuitive solutions
        match player.rx.recv().unwrap() {
            ReceiverData::Packet(mut pack)  => {
                if player.health > 0 || pack.id == 0x16 {
                    match packet_handler(&mut player, &mut pack) {
                        Some(_) => (),//kick_player(e),
                        _ => (),
                    }
                }
            },
            ReceiverData::KeepAlive => { let _ = player.stream.write(&[0x2u8, 0x0u8, 0x0u8]); },
            ReceiverData::TcpErr => return,
        }
    }
}

use struct_types::Location;
use to_client;
use packet::Packet;
use player::Player;
fn recv_keep_alive() -> Option<&'static str> {
    None
}
fn chat_message(player: &Player, packet: &mut Packet) -> Option<&'static str> {
    unimplemented!();
}
fn use_entity(player: &mut Player, packet: &mut Packet) -> Option<&'static str> {
    let target_id = packet.get_varint();
    let interact_type = packet.get::<u8>();
    if interact_type == 2 {
        let interact_location = packet.get_location();
        if player.location.distance(&interact_location) > 25.0 {
            Some("overexteding reach")
        } else {
            unimplemented!();
        }
    } else {
            None
    }
}
fn is_flying() -> Option<&'static str> {
    None
}
fn position_update(player: &mut Player, packet: &mut Packet) -> Option<&'static str> {
    //Fixe me
    let new_pos = packet.get_location();
    println!("{0} XYZ {1:.2} {2:.2} {3:.2}",player.name, new_pos.x, new_pos.y, new_pos.z);
    let on_ground = packet.get::<bool>();
    if player.location.distance(&new_pos) > 100.0 {
        return Some("moving to fast");
    } else {
        //Fix me, Check for closest  y block below to prevent fall damage avoiding
        let fall_dist = player.last_on_ground.y - new_pos.y - 3.0;
        if fall_dist > 0.0 && on_ground  {
            player.health -= fall_dist as i16;
            player.update_health();
            player.last_on_ground = new_pos;
        }
    }
    //player.location = new_pos;
    None
}

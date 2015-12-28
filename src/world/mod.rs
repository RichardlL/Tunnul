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


// --This module will handle all logic for map data, including--
//
// Deciding whether or not to send map data, or tell client to deallocate chunk
// Legalilty of a map modification/ player movement
// I/O to and from client , file modifcation, saving, etc.
// Map generation
//
//
// mod load;
// mod generate;
//
// use player::{Player, Location};
// use packet::Packet;
//
//
// impl Player {
// For packets 0x04 and 0x06
// fn update_position(&mut self, p: &mut Packet) {
// 0x4 updates position, 0x6 updates pos and look
// let new_location =  match p.id {
// 0x4 => Location {
// x: p.get::<f64>(),
// y: p.get::<f64>(),
// z: p.get::<f64>(),
// yaw: p.get::<f32>(),
// pitch: p.get::<f32>(),
// },
// 0x6 => Location {
// x: p.get::<f64>(),
// y: p.get::<f64>(),
// z: p.get::<f64>(),
// yaw: self.location.yaw,
// pitch: self.location.pitch,
// },
// _ => panic!("update_pos called with wrong id: {}",p.id),
// };
// if self.location.distance(&new_location) > 100.0 {
// panic!("{} Kicked for moving too quick", self.name);
// }
// Chunk difference x and y for loading/unloading
// let  (x_chunk_old, z_chunk_old, x_chunk_new, z_chunk_new) = (
// (self.location.x as i64) / 16,
// (self.location.z as i64) / 16,
// (new_location.x as i64) /16,
// (new_location.z as i64) /16,
// );
// let x_dif = x_chunk_old - x_chunk_new;
// let z_dif = z_chunk_old - z_chunk_new;
// Distances will be either -1 or 1, We will take the oppisite,
// and multiply by load distance to figure out
// which chunk to unload relative to the player's old position.
// When we find out which X to unload, we unload all of them ( regardless of their y values)
// (Minecraft doesnt load in a circle, but a square pattern)
// if x_dif != 0 {
// let unload_x = x_chunk_old + (load_circle * -x_dif);
// let z_range = (-load_dist..load_dist+1);
// for unload_z in zrange {
// p.unload_chunk(unload_x, unload_z);
// }
// }
// if z_dif != 0 {
// let unload_z = z_chunk_old + (load_circle * -z_dif);
// for unload_x in (-load_dist..load_dist+1) {
// p.unload_chunk(unload_x, unload_z);
// }
// }
// }
// }
//
//

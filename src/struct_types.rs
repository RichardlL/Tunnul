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

pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Location {
    pub fn new() -> Location {
        Location {
            x: 15.0,
            y: 60.0,
            z: 88.0,
        }
    }
    pub fn form_postition(&self) -> u64 {
        (((self.x as u64) & 0x3FFFFFFu64) << 38) |
        ((self.z as u64) & 0x3FFFFFFu64) |
        (((self.y as u64) & 0xFFFu64) << 26)
    }
    // Note - the actual distance is the square root of this, this is simply for hacking, etc
    pub fn distance(&self, loc: &Location) -> f64 {
        (self.x - loc.x).powi(2) + (self.z - loc.z).powi(2) + (self.z - loc.z).powi(2)
    }
}
use packet::Packet;
impl Packet {
    // NOTE: minecraft protocol is wack.
    // Some use f32, most use f64
    pub fn get_location(&mut self) -> Location {
        Location { 
            x: self.get::<f64>(),
            y: self.get::<f64>(),
            z: self.get::<f64>(),
        } 
    }
}
use std::clone::Clone;
impl Clone for Location {
    fn clone(&self) -> Location {
        Location {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
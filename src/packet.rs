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
// -------------------------------------------------------------------------
// THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT,
// NEITHER APPROVED NOR ASSOCIATED WITH MOJANG.


// Note: All packets sent after login will be in player.rs

// Packet id is defined by minecraft so you know how to handle the
// packet e.g. block updates, movement, etc.
// Data is just the rest of the packet, and will vary based on id
use std::net::TcpStream;
use std::io::Read;
use std::vec::IntoIter;
//use primitive::read_varint;
use std::time::Duration;
use primitive::VarInt;
use std::mem;
pub struct Packet {
    pub id: u8,
    pub data: IntoIter<u8>,
}
impl Packet {
    // Takes a tcp stream and pulls a packet from it
    // MAJOR FIX : no guarantee of full packet
    pub fn new(mut stream: &mut TcpStream) -> Result<Packet, &'static str> {
        let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
        
        let mut data = match stream
                .bytes()
                .map(|i| i.unwrap())
                .get_varint() as usize
                {
            len @ 1...1024 => vec![0; len],
            _ => return Err("Packet Forming error: Packet wrong \
                             size or stream Terminated"),
        };
         
        let mut pack = match stream.read_exact(&mut data) {
            Ok(_) => Packet {
                id: unsafe { mem::uninitialized() },
                data: data.into_iter()
            },
            Err(_) => return Err("Error reading Packet"),
        };
        pack.id = pack.get_varint() as u8;
        Ok(pack)
    }
}
use std::sync::mpsc::Sender;
use std::sync::mpsc::SendError;
use player_loop::ReceiverData;

pub fn form_packet(
    mut stream: Box<TcpStream>,
    tx: Sender<ReceiverData>
	) -> Result<(), SendError<ReceiverData>> {
    loop {
        match Packet::new(&mut stream) {
            Ok(p) => try!(
                tx.send(ReceiverData::Packet(p))
            ),
            Err(_) => return tx.send(ReceiverData::TcpErr),
        }
    }
}




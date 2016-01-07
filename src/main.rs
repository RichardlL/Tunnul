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


#![feature(ip_addr)]
#![feature(slice_patterns)]
#[macro_use]

// Conversion to minecraft types and sending them
mod packet_sending;

// Conversion from minecraft types
mod primitive;

//
mod new_connection;

// Player loop, packet handling, and player data, player actions
mod player;
mod player_loop;

// location type
mod struct_types;

// Data Tramsfer
use std::net::{TcpListener, TcpStream};

// Packet decoding and encoding, connection handling
mod packet;

// World loading
mod world;

// Contains most packets sent to client
mod to_client;

// Map Saving/loading
use std::path::Path;
use std::fs;

// multi-threading - used all over
use std::thread;
use std::sync::mpsc::channel;

// Keep Alive loop
mod keep_alive;
fn main() {
    println!("Starting Tunul");
    let socket = match TcpListener::bind("127.0.0.1:25565") {
        Ok(x) => x,
        Err(_) => {
            println!("Error Binding, Do you have permission, or is another process running?");
            return;
        },
    };
    println!("Bound Server Successfully, Open for Connections");

    // we'll have a seperate thread that handles all of the keep alives sends
    // (server has to ping client every 20 seconds,)
    // but well let each client's thread handle the response,
    // so it will know when a client disconnects
    let (keep_alive_tx, keep_alive_rx) = channel();
    thread::spawn(move || keep_alive::keep_alive_loop(keep_alive_rx));
    for stream in socket.incoming() {
        let keep_alive_tx_clone = keep_alive_tx.clone();
        let stream = Box::new(stream.unwrap());
        thread::spawn(move || new_connection::new_connection(stream, keep_alive_tx_clone));
        
    }
}

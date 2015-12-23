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


#![feature(ip_addr)]
#![feature(slice_patterns)]
#[macro_use]
mod packet_sending;

mod conversion;      // Conversion to and from minecraft's format.
                     // Nothing too interesting here, besides the
                     // algorithms, which are probably bad examples;
                     // anything that uses this will have to use:
                     // conversion:: directly
                     // or itt:: (iterators conversion)


// Player loop, packet handling, and player data, player actions
mod player;
mod player_loop;
// Data Tramsfer
use std::net::{TcpListener, TcpStream};

// Packet decoding and encoding, connection handling
mod packet;
// World loading
mod world;

// Map Saving/loading
use std::path::Path;
use std::fs;

// multi-threading - used all over
use std::thread;
use std::sync::mpsc::channel;

// Spawns Threads for connections, and hands off to new_connection
//  to decide if its ping or to join game
use std::sync::{Arc, Mutex};
fn main() {
    println!("Starting Tunul  ");
    let socket = match TcpListener::bind("127.0.0.1:25565") {
        Ok(x) => x,
        Err(_) => panic!("Error Binding, Do you have permission, or is another process running?"),
    };
    println!("Bound Server Successfully, Open for Connections");

    // we'll have a seperate thread that handles all of the keep alives sends
    // (server has to ping client every 20 seconds,)
    // but well let each client's thread handle the response,
    // so it will know when a client disconnects
    let (keep_alive_tx, keep_alive_rx) = channel();
    thread::spawn(move || packet::keep_alive_loop(keep_alive_rx));
    for stream in socket.incoming() {
        let stream = Box::new(stream.unwrap()) ;
        let _ = keep_alive_tx.send(stream.try_clone().unwrap());
        thread::spawn(move || packet::new_connection(stream));
    }
}

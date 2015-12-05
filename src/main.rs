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

/*------------------------------------------------------------------/
| ORGINAZATION                                                      /
|-------------------------------------------------------------------/
 The (main file's) module imports explain what each file does and
 its use in the project, in an attempt to make this program easily
 navigate-able reader friendly

 All modules written in this project be explained here,
 even if they are not needed in main.rs, they'll just be
 commented out

 Linebreaks will be used liberally to group "pieces of logic",
 like a period does for for a complete thought with a sentence,
 or at discretion to improve readability

-------------------------------------------------------------------*/
#![feature(ip_addr)]
#![feature(slice_patterns)]


mod conversion;      // Conversion to and from minecraft's format.
use conversion::itt; // Nothing too interesting here, besides the
                     // algorithms, which are probably bad examples;
                     // anything that uses this will have to use:
                     // conversion:: directly
                     // or itt:: (iterators conversion)


// Player loop, packet handling, and player data, player actions
mod player;
// Data Tramsfer

use std::net::{TcpListener, TcpStream};
use std::slice::Split;

//names, chat
use std::{str,string};


//Packet decoding and encoding, connection handling
mod packet;

// multi-threading - used all over
use std::thread;
use std::sync::mpsc::channel;


// Spawns Threads for connections, and hands off to new_connection
//  to decide if its ping or to join game
fn main() {
        println!("Starting Tunul ");
        let socket = match TcpListener::bind("127.0.0.1:25565") {
                Ok(x) => x,
                Err(_) => panic!("Error Binding, Do you have permission, or is another process running?" ),
        };

        println!("Bound Server Successfully, Open for Connections");
        
        // we'll have a seperate thread that handles all of the keep alives sends
        // (server has to ping client ever 20 seconds,)
        // but well let each client's thread handle the response, so it will know when a client disconnects
        let (keep_alive_tx, keep_alive_rx) = channel();
        thread::spawn(move|| {packet::keep_alive_loop(keep_alive_rx)});
        
        for connection in socket.incoming() {
                let connection = connection.unwrap();
                let _ = keep_alive_tx.send(connection.try_clone().unwrap());
                thread::spawn(move|| { packet::new_connection(connection) } );
        }
}



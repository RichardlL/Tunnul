/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * This Source Code Form is "Incompatible With Secondary Licenses", as
 * defined by the Mozilla Public License, v. 2.0.
 */

extern crate num;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::Read;
fn main() {
        let connections = TcpListener::bind("127.0.0.1:25565").unwrap();
        for stream in connections.incoming() {
                thread::spawn(move|| {newplayer_handler(stream.unwrap())});
        }
}
fn newplayer_handler(stream: TcpStream) {
        stream.set_read_timeout(Some(Duration::new(30, 0)));
        stream.set_read_timeout(None);
        let mut stream = stream;
        let mut buff: Vec<u8> = Vec::new();
        let arr:[u8,20]; = tmp
        loop {
                stream.read(&mut buff).unwrap();
                let (length, size)= from_varint(&buff);
                let (pack_id, size1) = from_varint(&buff[size+1..]);
                let data = &buff[(length as usize) - size1 +1 ..length as usize];
              //  println!(
                //if size !=  length as usize - size as usize {
                //        println!("error Length:{} Size{}: Actual:{}",length,size,total_size);
               // } else {
                 //       println!("success");
                //}
        }
}
    
}
// takes client packets, and decides what to do with them
// We cant use StD from big edian since its a varint
// Confusing? It was confusing to write
// You would think this needs error handling, but it doesnt
// Funny how that works.
fn from_varint(src_array: &[u8])-> (i64, usize) {
        let mut result:i64 = 0;
        let mut vi_size:usize = 0;
        let pl = src_array.len();
        while vi_size < pl {
                println!("run:{} ",vi_size);
                //result |=   p's first seven digits  shifted all the way left
                result |= (src_array[vi_size] as i64 & 0xFF )  << (57 - (7 * vi_size)) ;  //25 - 7v is equal to 32 - 7 * (vi_size -1)
                // if first bit is 0
                if src_array[vi_size] & 0x80 == 0 {
                        println!("Hello vi:{},",vi_size);
                        break;
                }
                vi_size += 1;
        }
        vi_size += 1;
        // total number of 7 bytes read
        // We dont need to account for twos complement (see wikipedia) , i32 bitshift already does
        result = result >> (32 - (7 * vi_size ));
        (result, vi_size)
}

 // the 9  / 8 is neccesary, since we need 1 bit to
                // tell if we have byte. the 7 always makes it 
                // rounds up unless its a round number 


 // puts 1 on the end of each bit and adjusts to account, and only returns the needed array
fn to_varint(src: i64) -> Box<[u8]> {
        let src_size = std::mem::size_of::<i64>() * 8;
        let plus_num = |x, y| -> Box<[u8]> {
                let bytes:usize = ((src_size - (src.leading_zeros() as usize)) * 9  + 7) / 8;
                let mut result: Vec<u8> = vec![0; bytes];;
                for i in 0..bytes+1 {
                        result[10-i] = (y ^ (x << i)) as u8 ;
                }
                result[10] &= 0xFF;
                result.into_boxed_slice()
        };
        let firstdigit = 0x1 << 63 as i64;
        match src  & firstdigit {
                0x100000000  => plus_num(src ^( 0x100000000 as i64 >> 63),0x1FF),
                _  => plus_num(src,0x100),
        }
}
/*
fn handle_packet(packet: &[u8]) -> () {
        let (packet_size, vi_size)= from_varint(packet);
        if  packet_size + vi_size as i32 != (*packet).len() as i32 {
                println!("Error, packetsize wrong");
        }
        let (packet_id, vi_size) = from_varint(&packet[vi_size+1..]);
        let p = [from_varint];
        let x:[u8; 4] = unsafe { std::mem::transmute(1)};
        p[packet_id as usize](&x);
}



*/




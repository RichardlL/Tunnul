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


// Map Module
// ---------------------------------------------------------------------------------------
// A "block_line" is a sequential list of blocks, eg (0, grass, 15)  15 sequestianl blocks
// the numbers are there absoulte position, as apposed to block xyz, so, 
// abs = (y * 225 + x * 15 + z)

// "block" when there are empty spaces in block lines.
// e.g. with the example above, the first space would be 16. 
// This way we dont have to use block positions

// Blocks line wil be used when they will save data. (3 or more of the same blocks in a row)
// Blocks will fill in empty points in the data

// id_point points to the actual id/meta in the unique block array. This is so we can save
// Data, and just can save data and since we can make "id_point smaller"

// unique_block is the array the id points to

// Block lines can cover each other. The one highest in the array will be sent. 
// e.g. 0-16 grass 5-9 dirt
// means grass will go from 0-4 and dirt from 5-8 and grass 9-15
/*
use std::mem::transmute;
use std::net::TcpStream;
use std::slice::from_raw_parts;
use std::io::Write;
use packet_sending::Var32;

struct BlockLine {
    start: u16,
    end: u16,
    id: u16,
}
struct WorldChunk {
    x:i32
    z:i32
    block_lines: Vec<BlockLine>,
    single_block: Vec<u16>,
}

fn xyz(x: u16, y: u16, z: u16) -> u16 {
    (y * 225) + (z * 15) + x
}
impl BlockLine {
    fn write(&self, stream: &mut TcpStream) {
        let p: *const u8 = unsafe { transmute(self) };
        let arr = unsafe { from_raw_parts(p, 2) };
        stream.write(&arr);
    }
}
impl WorldChunk {

	// a minimal generic world chunk.
	// Bedrock, stone, ores
    fn new_generic(&x: i32, &y: i32) -> WorldChunk {
        let bed_rock_line = BlockLine {
            start: xyz(0,0,0),
            end: xyz(0,2, 0),
            id: 7 << 4,
        };
        let stone = BlockLine {
            start: bed_rock_line.end + 1,
            end: xyz(0,63,0),
            id: 1 << 4,
        };

        WorldChunk {
            block_lines: vec![bed_rock_line, stone],
            single_block: Vec::new()
            x: x
            y: y
        }
    }
    fn send(&self, stream: &mut TcpStream) {
        let mut position = 0;
        let mut single_block_pos = 0;
        loop {
            let block =  match self.block_lines
                .iter()
                .rposition(|b_line| b_line.end > position && b_line.start <= position) {
                    Some(s) => s,
                    None => match 0 {
                        _ if self.single_block.len() > single_block_pos => {
                            stream.write(&unsafe {transmute::<_,[u8;2]>(self.single_block[single_block_pos])});
                            single_block_pos += 1;
                            position += 1;
                            continue;
                        },
                        _ => break,
                    }
            };
            let end = match self.block_lines[block+1..]
                .iter()
                .rev()
                .find(|b_line| b_line.start < self.block_lines[block].end) {
                    Some(b_line) => b_line.start,
                    None => self.block_lines[block].end,
            };
            for _ in position..end {
                    self.block_lines[block].write(stream);
            }
            position = end;
        }
        for _ in 0..(65536 - position) * 2 {
            stream.write(&[0u8;2]);
        }
    }
}
fn test_map(stream: &mut TcpStream) {
    let chunk = WorldChunk::new_generic(0, 0);
    Send! { stream: 0x0u8, 1u8, 0i32, 0i32, 255u8, chunk,
}


 */



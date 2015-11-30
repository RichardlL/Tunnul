// Note: All packets sent after login will be in player.rs

// Packet id is defined by minecraft so you know how to handle the
// packet e.g. block updates, movement, etc.
// Data is just the rest of the packet, and will vary based on id
pub struct Packet {
        pub id: usize,
        pub data: Vec<u8>,
        pub index: usize,
}

// rather than just a vector, well also store a index, so we dont have
// to keep track of what we have already read

use std::time::Duration;
use std::net::TcpStream;
use conversion;
use std::io::Read;
use std::borrow;
impl Packet {
        //Takes a tcp stream and pulls a packet from it
        //MAJOR FIX : no guarantee of full packet
        //MAJOR FIX: prevent over allocation
        pub fn new(stream: &mut TcpStream) -> Packet {
                let mut stream = stream;
                let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));

                let (length, _) = conversion::itt::read(stream);
                let (packetid , sizeof_packetid) = conversion::itt::read(stream);
                let length_to_read = (length as usize) - sizeof_packetid;
                let mut buff:Vec<u8> = vec![0;length_to_read];
                stream.read(&mut buff);
                Packet { id: packetid as usize, data: buff,  index: 0 }
        }
        // Gets varint from current index position and updates index
        pub fn get_varint(&mut self) -> i32 {
                let (result, bytes) = conversion::varint::from((&self.data[self.index..self.data.len()]));
                self.index += bytes;
                result
        }
        //gets string from current index and updates position
        pub fn get_string(&mut self) -> borrow::Cow<str> {
                let size = (self.get_varint()+1) as usize;
                String::from_utf8_lossy(&self.data[self.index..size])
        }
}

// Checks if client wants the server's status or to join
// player_connect goes to player/mod.rs

// Were spawning a new thread rather than using this one
// to release the stream (streams are copies),
// and to release the first packet
use std::thread;
use player;
pub fn new_connection(stream: TcpStream) {
        let mut stream = stream;
        let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));

        let new_player_packet = Packet::new(&mut stream);
        let _ = match new_player_packet {
                //Packet { id: 0 , data: d, index:_} if d.is_empty() => {}, //FEATURE new_player_packet.ping_response(),

                Packet { id:0, ..} =>  thread::spawn(move|| {player::player_connect(new_player_packet, stream)}),

                Packet{..} => panic!("Malformed login packet"),
        };
}
pub fn send_status(stream: TcpStream) {
        unimplemented!();
}
use std::io::Write;
use std::time;
pub fn wrong_version(mut stream :TcpStream, client: u8, server: u8) {
        let client = client.to_string();
        let server = server.to_string();
        
        stream.set_write_timeout(Some(time::Duration::new(10, 0)));
        let message = ["{\"text\": \"Incompatable client (Are you using a beta or old version?)".as_bytes(),
          (",\n Your Protocol Version is ").as_bytes(),
          client.as_bytes(),
          ("\n Server verrsion: ").as_bytes(),
          server.as_bytes(),
          ("\"}").as_bytes(),
        ];
        let mut message_length:usize = 0;
        for i in &message {
                message_length += i.len();
                println!("i: {}",i.len());
        }
        let message_length_var = conversion::varint::to((message_length) as i32);
        let packet_length = conversion::varint::to((message_length_var.len() as i32)+ (message_length as i32) + 1);
        stream.write(&packet_length);
        stream.write(&[0x00]);
        stream.write(&message_length_var);
        for i in &message {
                stream.write(i);
        }
}

pub fn form_packet(mut stream: &TcpStream, data: &[&[u8]], packetid: u8) {
        let mut data_length:usize = 0;
        for c in data {
                data_length += c.len();
        }
        let packet_length = conversion::varint::to((1 + data_length)as i32);
        println!("packetlenth {}", data_length+1);
        stream.write(&*packet_length);
        stream.write(&[packetid]);
        for w in data {
                stream.write(*w);
        }
}

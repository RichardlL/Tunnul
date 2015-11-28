// Packet id is defined by minecraft so you know how to handle the
// packet e.g. block updates, movement, etc.
// Data is just the rest of the packet, and will vary based on id
struct Packet {
        id: usize,
        data: PacketData,
}

// rather than just a vector, well also store a index, so we dont have
// to keep track of what we have already read
struct PacketData {
        array: Vec<u8>,
        index: usize,
}

impl Packet {
        //Takes a tcp stream and pulls a packet from it
        fn new(stream: &mut TcpStream) -> Packet {
                let mut stream = stream;

                let (length, _) = itt::read(stream);
                let (packetid , sizeof_packetid) = itt::read(stream);
                //FIX : prevent over allocation
                let mut buff: Vec<u8> = Vec::with_capacity((length as usize) - sizeof_packetid);
                let _ = stream.read(&mut buff);

                Packet { id: packetid as usize,
                         data: PacketData{ array: buff, index: 0 } }
        }
        // Gets varint from current index position and updates index
        fn get_varint(&self) -> i32 {
                let (result, bytes) = conversion::varint::from(&self.data.array[self.data.index..]);
                self.array.index += bytes;
                result
        }
}

// Checks if client wants the server's status or to join
// player_connect goes to player/mod.rs

// Were spawning a new thread rather than using this one
// to release the stream (streams are copies),
// and to release the first packet
fn new_connection(stream: TcpStream) {
        let mut stream = stream;
        let _ = stream.set_read_timeout(Some(Duration::new(20, 0)));

        let new_player_packet = Packet::new(&mut stream);
 /*       match new_player_packet {
                Packet { id: 0 , data: d } if d.is_empty() => {}, //FEATURE new_player_packet.ping_response(),

                Packet { id:0, ..} =>  thread::spawn(move|| {
                               player_connect(new_player_packet, stream)
                }),

                _ => Panic!("Malformed login packet"),
        }
*/
}


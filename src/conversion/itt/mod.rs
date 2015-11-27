use std::net::TcpStream;
use std::io::Read;

fn read(src_array:&mut TcpStream) -> (i64, usize) {
        let mut result:i64 = 0;
        let mut vi_size:usize = 0;
        let mut temp: u8;
        loop {
                temp = src_array.bytes().next().unwrap().unwrap();
                result |= ((temp & 0xFF ) as i64)  << (57 - (7 * vi_size)) ;
                if temp & 0x100u8 == 0 {
                        break
                }
                vi_size += 1;
        }
        vi_size += 1;
        (result >> (32 - (7 * (vi_size) )), vi_size)
}
// Array of packet handler, instead of doing a huge match
// We just take the packet ID, packet_handler[packet.id]
// (packet id 0 is position 0),
/*static packet_handler = [
        recv_keep_alive(),
        chat_message(),
        use_entity(),
        is_flying(),
        position_update(),
        look_update(),
        position_and_look_update(),
        player_digging(),
        block_placement(),
        held_item_update(),
        action(),
        vehichle_steer(),
        close_window(),
        click_inventory_slot(),
        confirm_transaction(),
        creative_inventory(),
        enchant_item(),
        sign_set(),
        player_abilities(),
        tab_complete(),
        client_settings(),
        spawn_request(),
        plugin_message(),
        spectate(),
        resource_pack_status(),
        ];
*/
use player;
use packet;
use std::net::TcpStream;
pub fn player_loop(mut stream:  Box<TcpStream>) {
    let mut player = player::Player::from_stream(stream);

    player.confirm_login();
    player.join_game();
    player.send_spawn();
    player.send_location();
    loop {}
}
    loop {
        let packet = packet::Packet::new(&mut stream);
        match packet.id {
            0..packet_handler.len() => (),
            _ => player.kick_player("Invalid Packet ID,  Hacking?"),
        }
        match player.packet_handler[packet.id](&packet) {
            Ok => (),
            Err(e) => { player.kick_player(e); return; },
        }
    }
}

impl Player {
    recv_keep_alive(&packet) -> Result {
        Ok
    }
    fn chat_message(&packet) {
        let message = try!(inputpacket.get_string());
    }
}x


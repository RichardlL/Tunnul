use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;
use player_loop::ReceiverData;

// checks for incoming player tx's
// If write fails (stream closed), it stops sending keep alive

pub fn keep_alive_loop(rx: Receiver<Sender<ReceiverData>>) {
    let mut connections = Vec::new();
    loop {
        if let Ok(keep_alive_tx) = rx.try_recv() {
            connections.push(keep_alive_tx);
        }
        let mut i = connections.len();
        while i != 0 {
            i -= 1;
            if connections[i].send(ReceiverData::KeepAlive).is_err() {
                connections.swap_remove(i);
            }
        }
        sleep(Duration::from_secs(15));
    }
}
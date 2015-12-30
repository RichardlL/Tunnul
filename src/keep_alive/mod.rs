use std::sync::mpsc::{Sender, Receiver};
use std::thread::sleep;
use std::time::Duration;
use player_loop::ReceiverData;
// checks for incoming player tx's
// If write fails (stream closed), it stops sending keep alive

pub fn keep_alive_loop(rx: Receiver<Sender<ReceiverData>>) {
    let mut connections: Vec<Sender<ReceiverData>> = Vec::new();
    loop {
        if let Ok(keep_alive_tx) = rx.try_recv() {
            connections.push(keep_alive_tx);
        }
        let mut p = connections.len();
        while p != 0 {
            p -= 1;
            if connections[p].send(ReceiverData::KeepAlive).is_err() {
                connections.swap_remove(p);
            }
        }
        sleep(Duration::from_secs(15));
    }
}
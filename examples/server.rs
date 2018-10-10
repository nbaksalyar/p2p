#![allow(deprecated)]

extern crate mio;
extern crate p2p_old;
extern crate rust_sodium as sodium;
extern crate serde_json;
#[macro_use]
extern crate unwrap;

use self::event_loop::spawn_event_loop;
use p2p_old::{NatMsg, TcpRendezvousServer, UdpRendezvousServer};
use std::sync::mpsc;

mod event_loop;

fn main() {
    let el = spawn_event_loop();
    unwrap!(el.nat_tx.send(NatMsg::new(move |ifc, poll| {
        let _token_udp = unwrap!(UdpRendezvousServer::start(ifc, poll));
        let _token_tcp = unwrap!(TcpRendezvousServer::start(ifc, poll));
    })));

    let (_tx, rx) = mpsc::channel();
    println!("Server started. Blocking main thread indefinitely");
    unwrap!(rx.recv())
}

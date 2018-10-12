#![allow(deprecated)]

#[macro_use]
extern crate log;
extern crate maidsafe_utilities;
extern crate mio;
extern crate p2p;
extern crate rust_sodium as sodium;
extern crate serde_json;
#[macro_use]
extern crate unwrap;
extern crate socket_collection;

use self::event_loop::spawn_event_loop;
use maidsafe_utilities::log as logger;
use p2p::{NatMsg, TcpRendezvousServer, UdpRendezvousServer};
use std::sync::mpsc;

mod event_loop;

fn main() {
    unwrap!(logger::init(false));

    let el = spawn_event_loop();
    unwrap!(el.nat_tx.send(NatMsg::new(move |ifc, poll| {
        let _token_udp = unwrap!(UdpRendezvousServer::start(ifc, poll));
        let _token_tcp = unwrap!(TcpRendezvousServer::start(ifc, poll));
    })));

    let (_tx, rx) = mpsc::channel();
    println!("Server started. Blocking main thread indefinitely");
    unwrap!(rx.recv())
}

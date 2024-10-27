// Build:
//
// ```
// cd netlink-sys
// cargo run --example listen_uevents
//
// ```
//
// Run *as root*:
//
// ```
// find /sys -name uevent -exec sh -c 'echo add >"{}"' ';'
// ```
//
// To generate events.

use std::process;

use netlink_sys::{protocols::NETLINK_KOBJECT_UEVENT, Socket, SocketAddr};

use kobject_uevent::UEvent;

fn main() {
    let mut socket = Socket::new(NETLINK_KOBJECT_UEVENT).unwrap();
    let sa = SocketAddr::new(process::id(), 1);

    socket.bind(&sa).unwrap();

    while let Ok((buf, _addr)) = socket.recv_from_full() {
        let s = std::str::from_utf8(&buf).unwrap();
        let u = UEvent::from_netlink_packet(&buf).unwrap();
        println!(">> {}", s);
        println!("{:#?}", u);
    }
}

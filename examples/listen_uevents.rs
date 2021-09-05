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
    let mut buf = vec![0; 1024 * 8];

    socket.bind(&sa).unwrap();

    loop {
        let n = socket.recv(&mut buf, 0).unwrap();
        let s = std::str::from_utf8(&buf[..n]).unwrap();
        let u = UEvent::from_netlink_packet(&buf[..n]).unwrap();
        println!(">> {}", s);
        println!("{:#?}", u);
    }
}

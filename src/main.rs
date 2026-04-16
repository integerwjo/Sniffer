use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{self, PacketSize};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};

fn main() {
    let interfaces = datalink::interfaces();

    for interface in interfaces {
        println!("{}", interface.name);
    }

    let interface = datalink::interfaces()
        .into_iter()
        .find(|interface| interface.name == "eth0")
        .expect("Failed to find the eth0 interface");
        
    println!("Found interface: {}", interface.name);

    let mut rx = match datalink::channel(&interface,  Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error creating channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                println!("Received packet: {}", packet.len())
            }
            Err(e) => {
                println!("Error receiving packet: {}", e)
            }
        }
    }


}
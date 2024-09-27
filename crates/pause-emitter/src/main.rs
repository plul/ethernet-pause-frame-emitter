//! Ethernet PAUSE frame emitter.

use clap::Parser;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::Packet;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::util::MacAddr;

/// Command line options
#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long)]
    interface_name: String,

    #[arg(long, default_value = "10ms")]
    interval: humantime::Duration,

    #[arg(long, default_value_t = u16::MAX)]
    pause_time: u16,
}

fn main() {
    tracing_subscriber::fmt::init();
    let Cli {
        interface_name,
        interval,
        pause_time,
    } = Cli::parse();

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.iter().find(|iface| iface.name == interface_name).unwrap_or_else(|| {
        panic!(
            "No interface with name {interface_name} found in interfaces: {}",
            interfaces.iter().map(|iface| iface.name.clone()).collect::<Vec<_>>().join(", ")
        )
    });

    tracing::info!("Interface: {interface}");

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, _rx) = match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(err) => panic!("An error occurred when creating the datalink channel: {err}"),
    };

    loop {
        // Create a PAUSE frame
        let mut buffer = [0u8; 60]; // Minimum Ethernet frame size
        let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap();

        // Set Ethernet header
        ethernet_packet.set_destination(MacAddr::new(0x01, 0x80, 0xC2, 0x00, 0x00, 0x01));
        ethernet_packet.set_source(interface.mac.unwrap());
        ethernet_packet.set_ethertype(pnet::packet::ethernet::EtherType::new(0x8808));

        // Set PAUSE frame payload
        // Payload format: [Opcode (2 bytes), Pause time (2 bytes)]
        let mut pause_frame = [0u8; 4];
        const PAUSE_OPCODE: u16 = 0x0001;
        pause_frame[0..2].copy_from_slice(&PAUSE_OPCODE.to_be_bytes());
        pause_frame[2..4].copy_from_slice(&pause_time.to_be_bytes());

        ethernet_packet.set_payload(&pause_frame);

        tracing::info!("Sending PAUSE frame {ethernet_packet:?} with pause time {pause_time}");
        tx.send_to(ethernet_packet.packet(), None);

        std::thread::sleep(interval.into());
    }
}

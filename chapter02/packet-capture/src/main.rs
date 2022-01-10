use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
#[macro_use]
extern crate log;

use std::env;

mod packets;
use packets::GettableEndPoints;

const WIDTH: usize = 20;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }
    let interface_name = &args[1];

    /* インターフェースの選択 */ 
    // 現在のマシンで選択可能なネットワークインターフェースの一覧を取得する
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        // 一覧取得したネットワークインターフェースと引数に指定したネットワークインターフェース名が一致するものを取得
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");
}
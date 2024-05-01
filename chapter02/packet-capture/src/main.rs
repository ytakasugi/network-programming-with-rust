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
    let args = env::args().collect::<Vec<String>>();
    // 引数チェック
    if args.len() < 2 {
        error!("Please provide a file path");
        std::process::exit(1);
    }
    // ネットワークインターフェース名取得
    let interface_name = &args[1];
    // ネットワークインターフェース一覧
    let interfaces = datalink::interfaces();
    // ネットワークインターフェース名からデータリンクチャンネルを取得
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failt to get interface.");

    // データリンクチャンネルを作成
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    // 1. イーサネットフレームを構築
    // 2. IPv4パケットを構築
    // 3. TCP/UDPパケットを構築
    // 4. アプリケーション層の情報を表示
    // 
    // パケットの構造
    // --------------------
    // | イーサネットヘッダ |
    // |-------------------|
    // | IPヘッダ          |
    // |-------------------|
    // | TCP/UDPヘッダ     |
    // |-------------------|
    // | データ(ペイロード) |
    // ---------------------
    loop {
        match rx.next() {
            Ok(frame) => {
                // 受信パケットからイーサネットフレームを構築
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    }
                    _ => {
                        info!("Not an IPv4 or IPv6");
                    }
                }
            }
            Err(e) => {
                error!("Faild to read: {}", e);
            }
        }
    }
}

/// IPv4パケットを構築し、ペイロードから次のレイヤのハンドラを呼び出す
fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP/UDP packet");
            }
        }
    }
}

/// IPv6パケットを構築し、ペイロードから次のレイヤのハンドラを呼び出す
fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP/UDP packet");
            }
        }
    }
}

/// TCPパケットを構築する
fn tcp_handler(packet: &dyn GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

/// UDPパケットを構築する
fn udp_handler(packet: &dyn GettableEndPoints) {
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}

/// アプリケーション層の情報を表示する
fn print_packet_info(l3: &dyn GettableEndPoints, l4: &dyn GettableEndPoints, proto: &str) {
    println!(
        "Captured a {} packet from {}|{} to {}|{}\n",
        proto,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination()
    );
    let payload = l4.get_payload();
    let len = payload.len();

    // ペイロードの表示
    // 指定した定数幅で表示を行う
    for i in 0..len {
        print!("{:<02X} ", payload[i]);
        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            for _j in 0..WIDTH - 1 - (i % (WIDTH)) {
                print!("   ");
            }
            print!("| ");
            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    // 非ascii文字は.で表示
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}

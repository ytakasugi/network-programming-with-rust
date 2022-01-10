use std::env;

use pnet::datalink;

fn main() {
    let args: Vec<String> = env::args().collect();
    let interface_name = &args[1];

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        // 一覧取得したネットワークインターフェースと引数に指定したネットワークインターフェース名が一致するものを取得
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");
    
    //println!("{:?}", interfaces);
    println!("{}", interface);
}

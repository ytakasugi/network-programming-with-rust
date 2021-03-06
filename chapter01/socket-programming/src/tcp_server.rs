use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

// 指定のソケットアドレスで接続を待ち受ける。
pub fn serve(address: &str) -> Result<(), failure::Error> {
    // クライアントからのコネクション確率要求を待ち受ける
    let listener = TcpListener::bind(address)?;
    loop {
        // コネクション確率済みのソケット返却をする
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

pub fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        // `stream`にデータが流れてくるまで待機し、データが届いたら読み込んだバイト数を返す
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(())
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
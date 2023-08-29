
use std::net::SocketAddr;

use super::arg_parser::Args;

use tokio::{net::{TcpSocket, TcpStream}, io::AsyncWriteExt, io::AsyncReadExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};

pub async fn open(args : Args) {
    let addr = format!("{}:{}", args.server_ip, args.server_port);
    let mut stream = TcpStream::connect(&addr).await.unwrap();
    let (mut r,mut w) = stream.split();

    println!("Connected to server {}", addr);
    let mut buff = [0u8; 1024];
    loop {
        tokio::select! {
            n = r.read(&mut buff) => {
                match n {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }
                        println!("Received {} bytes", n);
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            },
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                println!("Sending data");
                let data = b"Hello world";
                w.write_all(data).await.unwrap();
            }
        }
    }
    println!("Connection closed");
}

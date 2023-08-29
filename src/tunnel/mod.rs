
use std::{collections::{HashMap, VecDeque}, vec, io::Read};
use super::arg_parser::Args;

use tokio::io::{self, ReadBuf, AsyncReadExt};
use tokio::net::{TcpStream, TcpListener, tcp::ReadHalf, tcp::WriteHalf};

const MAX_RECEIVE_BUFFER_SIZE:usize = 16384;
pub struct User {
    pub session_id: u32,
    pub id: u32,
    pub stream: TcpStream,
}
impl User {
    pub fn new(session_id:u32, id: u32, stream: TcpStream) -> Self {
        Self {
            session_id,
            id,
            stream,
        }
    }
}

pub async fn open(args: Args) {
    println!("Tunnel mode, port {}", args.server_port);

    let server_port :u16 = args.server_port.parse().unwrap();
    let addr = format!("0.0.0.0:{}", server_port);
    let listener = TcpListener::bind(addr).await.unwrap();

    let mut users:HashMap<i32, VecDeque<User>> = HashMap::new();
    let MAX_USER = 2;
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let session_id = 1234;
        let user_id = 1;
        if !users.contains_key(&session_id) {
            users.insert(
                session_id, 
                VecDeque::from(vec![User::new(session_id as u32, user_id, socket)])
            );
        }
        else {
            let v = users.get_mut(&session_id).unwrap();
            if !v.is_empty() {
                let cnt = v.len() + 1;
                let u1 = v.pop_front().unwrap();
                let u2 = User::new(session_id as u32, user_id, socket);
                if cnt >= MAX_USER {
                    start_relaying(u1, u2);
                    continue;
                }
            } else {
                v.push_back(User::new(session_id as u32, user_id, socket));
            }
        }
    }
}

async fn async_read(r_half: &ReadHalf<'_>, buff: &mut Vec<u8>) -> Result<usize, std::io::Error> {
    r_half.try_read(buff.as_mut())
}
async fn async_write(w_half: &mut WriteHalf<'_>, buff: &[u8]) -> Result<usize, std::io::Error> {
    w_half.try_write(buff)
}

fn start_relaying(user1:User, user2:User) {
    tokio::spawn(async move {
        let session_id = user1.session_id.clone();
        println!("Start relaying {}", session_id);
        let mut buff1 = vec![0u8; MAX_RECEIVE_BUFFER_SIZE];
        let mut buff2 = vec![0u8; MAX_RECEIVE_BUFFER_SIZE];
        let mut s1 = user1.stream;
        let mut s2 = user2.stream;
        let (mut r1, mut w1) = s1.split();
        let (mut r2, mut w2) = s2.split();
        
        println!("Relaying {}", session_id);
        loop {
            tokio::select! {
                // ret = async_read(&r1, buff1.as_mut()) => {
                ret = r1.read(buff1.as_mut()) => {
                    match ret {
                        Ok(size) => {
                            if size == 0 {
                                println!("s1 closed");
                                break;
                            }
                            println!("s1 {} bytes to s2", size);
                            async_write(&mut w2, &buff1[0..size]).await.unwrap();
                        },
                        Err(e) => {
                            println!("Error1 : {}", e);
                            break;
                        }
                    }
                },
                ret = r2.read(buff2.as_mut()) => {
                    match ret {
                        Ok(size) => {
                            if size == 0 {
                                println!("s2 closed");
                                break;
                            }
                            println!("s2 {} bytes to s1", size);
                            async_write(&mut w1, &buff2[0..size]).await.unwrap();
                        },
                        Err(e) => {
                            println!("Error2 : {}", e);
                            break;
                        }
                    }
                },
            }   
        }    
        println!("End relaying {}", session_id);
    });
}

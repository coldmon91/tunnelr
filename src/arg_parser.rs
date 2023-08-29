
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("tunnel"))]
    pub mode: String, 
    #[arg(short, long, default_value_t = String::from("22"))]
    pub local_port: String,

    // server
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub server_ip: String,
    #[arg(long, default_value_t = String::from("58080"))]
    pub server_port: String,
}

pub fn parse() -> Args {
    Args::parse()
}
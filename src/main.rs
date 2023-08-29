
use tokio;
mod arg_parser;
mod tunnel;
mod agent;

#[tokio::main]
async fn main() {
    let args = arg_parser::parse();
    println!("{:?}", args);
    let tunnel_port :u16 = args.server_port.parse().unwrap();
    if args.mode == "agent" {
        agent::open(args).await;
    } else if args.mode == "tunnel" {
        tunnel::open(args).await;
    } else {
        println!("Invalid mode");
    }
}

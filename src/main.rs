mod connection;
mod packet;

use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use log::info;
use rand_core;
use rand_core::RngCore;
use crate::connection::client::Client;

struct Context {
    pub connections: HashMap<SocketAddr, Client>
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut context = Context {
        connections: HashMap::new()
    };

    let mut private_key: [u8; 128] = [0; 128];
    let mut socket = UdpSocket::bind("127.0.0.1:22023").unwrap();
    info!("Game server running on {:?}", socket.local_addr().unwrap());
    rand_core::OsRng::default().fill_bytes(&mut private_key);

    tokio::spawn(async move {
        matchmaking::create_matchmaking_thread(private_key).await;
    });
    loop {
        let mut buf = [0; 2048];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        let buf = &mut buf[..amt];

        if !context.connections.contains_key(&src) {
            let socket_addr = src.clone();
            context.connections.insert(socket_addr, Client {
                net_address: socket_addr,
            });
        }

        info!("Data received: {:?} from {:?}", buf, src);
    }
}

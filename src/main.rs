use std::net::UdpSocket;
use log::info;
use rand_core;
use rand_core::RngCore;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
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
        info!("Data received: {:?} from {:?}", buf, src);
    }
}

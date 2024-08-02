#![allow(dead_code)]
mod enemies;
mod game_data;
mod handler;
mod packet;
mod player;
mod walls;

use crate::handler::ClientHandler;
use crate::packet::PacketTypes;
use multithreading::ThreadPool;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

fn main() {
    let server_ip = "127.0.0.1";
    let server_port = 5784;
    let socket = UdpSocket::bind(format!("{}:{}", server_ip, server_port))
        .expect("Failed to bind to socket");

    let mut buf = [0; 512];

    let pool = ThreadPool::new(num_cpus::get());

    let game_data = Arc::new(Mutex::new(game_data::GameData::new()));

    println!("Server listening at {}:{}", server_ip, server_port);
    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
        println!("Got a request from {}", src);
        println!("Data: {:?}", String::from_utf8_lossy(&buf[..amt]));
        let handler = ClientHandler::new(src, socket.try_clone().expect("Failed to clone socket"));
        let packet = packet::Packet::from_raw(buf[..amt].to_vec());

        // Clone the Arc to move into the closure
        let game_data = Arc::clone(&game_data);

        pool.execute(move || {
            let mut game_data = game_data.lock().expect("Failed to lock game data");

            let res = handler.on_packet(packet, &mut game_data);
            if let Err(e) = res {
                eprintln!("Error: {}", e);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_constants_string() {
        assert_eq!(format!("{}", PacketTypes::Join), "JOIN");
        assert_eq!(format!("{}", PacketTypes::ColorError), "COLOR_ERROR");
        assert_eq!(format!("{}", PacketTypes::LevelData), "LEVEL_DATA");
        assert_eq!(format!("{}", PacketTypes::Ready), "READY");
        assert_eq!(format!("{}", PacketTypes::StartGame), "START_GAME");
        assert_eq!(format!("{}", PacketTypes::Move), "MOVE");
        assert_eq!(format!("{}", PacketTypes::MatchEnded), "MATCH_ENDED");
        assert_eq!(format!("{}", PacketTypes::SetHealth), "SET_HEALTH");
        assert_eq!(format!("{}", PacketTypes::EnemyChange), "ENEMY_CHANGE");
        assert_eq!(format!("{}", PacketTypes::EnemyTp), "ENEMY_TP");
        assert_eq!(format!("{}", PacketTypes::Exit), "EXIT");
        assert_eq!(format!("{}", PacketTypes::UnknowType), "UNKNOWN_TYPE");
        assert_eq!(format!("{}", PacketTypes::Invalid), "INVALID");
        assert_eq!(format!("{}", PacketTypes::Ok), "OK");
        assert_eq!(format!("{}", PacketTypes::Error), "ERROR");
        assert_eq!(format!("{}", PacketTypes::Ping), "PING");
    }
}

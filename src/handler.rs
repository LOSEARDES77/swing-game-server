use std::net::SocketAddr;
use std::net::UdpSocket;

use crate::game_data::GameData;
use crate::packet::Packet;
use crate::player::Player;
use crate::PacketTypes;

#[derive(Debug)]
pub struct ClientHandler {
    address: SocketAddr,
    socket: UdpSocket,
}

impl Clone for ClientHandler {
    fn clone(&self) -> Self {
        Self {
            address: self.address,
            socket: self.socket.try_clone().expect("Failed to clone socket"),
        }
    }
}

impl ClientHandler {
    pub fn new(address: SocketAddr, socket: UdpSocket) -> Self {
        Self { address, socket }
    }

    pub fn send(&self, packet: Packet) {
        println!("Sending packet: {:?}", packet.get_type());
        println!("Data: {:?}", packet.get_data());
        println!("Raw data {:?}", packet.get_raw_data());
        self.socket
            .send_to(packet.get_raw_data().as_bytes(), self.address)
            .expect("Failed to send data");
    }

    pub fn handle_packet(&self, packet: Packet, game_data: &mut GameData) -> Result<(), String> {
        match packet.get_type() {
            PacketTypes::Join => {
                let packet_data = packet.get_data();
                let coolor = packet_data.split(';').collect::<Vec<&str>>();
                if coolor.len() != 3 {
                    self.send(Packet::new(
                        PacketTypes::ColorError,
                        "COLOR_ERROR".to_string(),
                    ));
                    return Err("Invalid color".to_string());
                }
                let r = coolor[0]
                    .parse::<u8>()
                    .map_err(|_| "Invalid color".to_string())?;
                let g = coolor[1]
                    .parse::<u8>()
                    .map_err(|_| "Invalid color".to_string())?;
                let b = coolor[2]
                    .parse::<u8>()
                    .map_err(|_| "Invalid color".to_string())?;
                let player = Player::new((r, g, b));
                game_data.add_player(self.address, player);

                self.send(Packet::new(PacketTypes::Ok, String::new()));
            }
            PacketTypes::SetColor => {}
            PacketTypes::ColorError => {}
            PacketTypes::ReadyUp => {}
            PacketTypes::LevelData => {}
            PacketTypes::Ready => {
                if let Some(player) = game_data.get_player(&self.address) {
                    let mut player = player.clone();
                    player.set_ready();
                    if player.is_ready() {
                        self.send(Packet::new(PacketTypes::Ok, "Ready".to_string()));
                    } else {
                        self.send(Packet::new(PacketTypes::Ok, "Not Ready".to_string()));
                    }
                    game_data.add_player(self.address, player);
                } else {
                    self.send(Packet::new(
                        PacketTypes::Error,
                        "Player not found".to_string(),
                    ));
                }
            }
            PacketTypes::StartGame => {}
            PacketTypes::Move => {}
            PacketTypes::MatchEnded => {}
            PacketTypes::SetHealth => {}
            PacketTypes::EnemyChange => {}
            PacketTypes::EnemyTp => {}
            PacketTypes::Exit => {}
            PacketTypes::UnknowType => {}
            PacketTypes::Invalid => {}
            PacketTypes::Ok => {}
            PacketTypes::Error => {}
            PacketTypes::Ping => {
                let time = packet.get_data().parse::<u128>().unwrap();
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                println!("Java time: {}", time);
                println!("Current time: {}", current_time);
                let ping = current_time - time;
                println!("Ping: {}", ping);
                self.send(Packet::new(
                    PacketTypes::Ping,
                    format!("{} {}", ping, current_time),
                ));
            }
        }

        Ok(())
    }
}

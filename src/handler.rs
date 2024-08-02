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

    pub fn on_packet(&self, packet: Packet, game_data: &mut GameData) -> Result<(), &str> {
        if !game_data.has_match_started() {
            match packet.get_type() {
                PacketTypes::Join => {
                    let packet_data = packet.get_data();
                    let coolor = packet_data.split(';').collect::<Vec<&str>>();
                    if coolor.len() != 3 {
                        self.send(Packet::new(PacketTypes::ColorError, "Invalid color"));
                        return Err("Invalid color");
                    }
                    let r = coolor[0].parse::<u8>().map_err(|_| "Invalid color")?;
                    let g = coolor[1].parse::<u8>().map_err(|_| "Invalid color")?;
                    let b = coolor[2].parse::<u8>().map_err(|_| "Invalid color")?;
                    let player = Player::new((r, g, b));
                    for player in game_data.get_players() {
                        if player.get_color() == (r, g, b) {
                            // TODO: Also filter similar colors
                            self.send(Packet::new(PacketTypes::ColorError, "Color alredy picked"));
                            return Err("Color already in use");
                        }
                    }
                    game_data.add_player(self.address, player);

                    self.send(Packet::new(PacketTypes::Ok, ""));
                }
                PacketTypes::ColorError => {} // Not supposed to receive this packet as the server
                PacketTypes::LevelData => {}  // Not supposed to receive this packet as the server
                PacketTypes::Ready => {
                    if let Some(player) = game_data.get_player(&self.address) {
                        let mut player = *player;
                        player.set_ready();
                        if player.is_ready() {
                            self.send(Packet::new(PacketTypes::Ok, "Ready"));
                        } else {
                            self.send(Packet::new(PacketTypes::Ok, "Not Ready"));
                        }
                        game_data.add_player(self.address, player);
                    } else {
                        self.send(Packet::new(PacketTypes::Error, "Player not found"));
                    }
                }
                PacketTypes::StartGame => {} // Not supposed to receive this packet as the server
                PacketTypes::Move => {}
                PacketTypes::MatchEnded => {} // Not supposed to receive this packet as the server
                PacketTypes::SetHealth => {}  // Not supposed to receive this packet as the server
                PacketTypes::EnemyChange => {} // Not supposed to receive this packet as the server
                PacketTypes::EnemyTp => {}    // Not supposed to receive this packet as the server
                PacketTypes::Exit => {}       // Not supposed to receive this packet as the server
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

                    let ping = current_time - time;
                    self.send(Packet::new(
                        PacketTypes::Ping,
                        format!("{} {}", ping, current_time).as_str(),
                    ));
                }
                PacketTypes::PlayerJoined => {
                    // Not supposed to receive this packet on the server
                }
            }
        } else {
            self.send(Packet::new(PacketTypes::Error, "Match not started"));
        }

        Ok(())
    }
}

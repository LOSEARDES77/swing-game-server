use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketTypes {
    Join,         // To join the game
    ColorError,   // If there is already a player with that color
    LevelData,    // The details of the level (walls, enemy quantity, ...)
    Ready,        // Marks the player as ready
    StartGame,    // Start the game
    Move,         // Move the player
    MatchEnded,   // When the match has concluded
    SetHealth,    // Change the health of any player
    EnemyChange,  // A change on an enemy (direction or axis)
    EnemyTp,      // An enemy teleported
    Exit,         // Finish
    UnknowType,   // Unknown type
    Invalid,      // Bad packet
    Ok,           // Everything is fine
    Error,        // Something went wrong
    Ping,         // Ping the server
    PlayerJoined, // A player joined the game
}

impl Display for PacketTypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                PacketTypes::Join => "JOIN",
                PacketTypes::ColorError => "COLOR_ERROR",
                PacketTypes::LevelData => "LEVEL_DATA",
                PacketTypes::Ready => "READY",
                PacketTypes::StartGame => "START_GAME",
                PacketTypes::Move => "MOVE",
                PacketTypes::MatchEnded => "MATCH_ENDED",
                PacketTypes::SetHealth => "SET_HEALTH",
                PacketTypes::EnemyChange => "ENEMY_CHANGE",
                PacketTypes::EnemyTp => "ENEMY_TP",
                PacketTypes::Exit => "EXIT",
                PacketTypes::UnknowType => "UNKNOWN_TYPE",
                PacketTypes::Invalid => "INVALID",
                PacketTypes::Ok => "OK",
                PacketTypes::Error => "ERROR",
                PacketTypes::Ping => "PING",
                PacketTypes::PlayerJoined => "PLAYER_JOINED",
            }
        )
    }
}

pub struct Packet {
    p_type: PacketTypes,
    data: String,
    raw_data: Vec<u8>,
    is_valid: bool,
}

impl Packet {
    pub fn from_raw(raw_data: Vec<u8>) -> Self {
        let mut type_string = String::from_utf8_lossy(&raw_data[..16]).to_string();
        type_string = type_string.trim().to_string();
        let p_type = match type_string.as_str() {
            "JOIN" => PacketTypes::Join,
            "COLOR_ERROR" => PacketTypes::ColorError,
            "LEVEL_DATA" => PacketTypes::LevelData,
            "READY" => PacketTypes::Ready,
            "START_GAME" => PacketTypes::StartGame,
            "MOVE" => PacketTypes::Move,
            "MATCH_ENDED" => PacketTypes::MatchEnded,
            "SET_HEALTH" => PacketTypes::SetHealth,
            "ENEMY_CHANGE" => PacketTypes::EnemyChange,
            "ENEMY_TP" => PacketTypes::EnemyTp,
            "EXIT" => PacketTypes::Exit,
            "UNKNOWN_TYPE" => PacketTypes::UnknowType,
            "INVALID" => PacketTypes::Invalid,
            "OK" => PacketTypes::Ok,
            "ERROR" => PacketTypes::Error,
            "PING" => PacketTypes::Ping,
            "PLAYER_JOINED" => PacketTypes::PlayerJoined,
            _ => PacketTypes::UnknowType,
        };
        let data_str = String::from_utf8_lossy(&raw_data[16..]).to_string();

        Self {
            p_type,
            data: data_str,
            raw_data: raw_data.clone(),
            is_valid: raw_data.len() <= 512,
        }
    }
    pub fn new(p_type: PacketTypes, data: &str) -> Self {
        let mut type_string = format!("{}", p_type);
        if type_string.len() < 16 {
            type_string.push_str(&" ".repeat(16 - type_string.len()));
        }
        let mut raw_data = type_string.into_bytes();
        raw_data.extend_from_slice(data.as_bytes());

        Self {
            p_type,
            data: data.to_string(),
            raw_data: raw_data.clone(),
            is_valid: raw_data.len() <= 512,
        }
    }

    pub fn get_type(&self) -> PacketTypes {
        self.p_type
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn get_raw_data(&self) -> String {
        String::from_utf8_lossy(&self.raw_data).to_string()
    }
}

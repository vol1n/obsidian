use once_cell::sync::Lazy;
use std::collections::HashMap;
use versions::mcp763::SERVERBOUND_PACKET_SCHEMAS;

use once_cell::sync::Lazy;
use std::collections::HashMap;

use schema::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PacketKey {
    id: u8,
    state: ConnectionState,
    direction: Direction,
}

static RAW_SCHEMA_JSON: &str = include_str!("versions/mcp763.json");

pub static PACKET_SCHEMA_MAP: Lazy<HashMap<PacketKey, PacketSchema>> = Lazy::new(|| {
    let parsed: Vec<PacketSchema> =
        serde_json::from_str(RAW_SCHEMA_JSON).expect("Invalid packet schema JSON");

    let mut map = HashMap::new();
    for schema in parsed {
        let key = PacketKey {
            id: schema.id,
            state: schema.state.clone(),
            direction: schema.direction.clone(),
        };
        map.insert(key, schema);
    }

    map
});

pub fn decode_varint(bytes: &Vec<u8>) -> Option<(u32, usize)> {
    let mut result: u32 = 0;
    let mut shift: i32 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        
        let val = (byte & 0x7F) as u32;
        result |= val << shift;

        if byte & 0x80 == 0 {
            return Some((result, i + 1)); // also return how many bytes consumed
        }

        shift += 7;
        if shift >= 35 {
            return None; // overflow / invalid VarInt
        }
    }
    None // not enough bytes yet
}

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

pub enum Direction {
    Serverbound,
    Clientbound
}

pub struct PacketKey {
    id: u16,
    state: ConnectionState,
    direction: Direction
}

pub static PACKET_SCHEMA_MAP: Lazy<HashMap<PacketKey, &'static PacketSchema>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for schema in SERVERBOUND_PACKET_SCHEMAS.iter() {
        let key = PacketKey {
            id: schema.id,
            state: schema.state,
            direction: schema.direction,
        };
        map.insert(key, schema);
    }

    map
});

pub fn decode_packet() {

}
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Direction {
    Serverbound,
    Clientbound,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FieldType {
    VarInt,
    String,
    U16,
    I64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: FieldType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PacketSchema {
    pub id: u8,
    pub name: String,
    pub kind: String, // or ServerboundPacketKind if you want enum
    pub state: ConnectionState,
    pub direction: Direction,
    pub fields: Vec<Field>,
}
#[macro_export]
macro_rules! serverbound_packet_table {
    ( $( ($id:expr, $state:ident) => $variant:ident $( ( $decoder:ident ) )? , )* ) => {
        fn decode_serverbound_packet(state: ConnectionState, packet_id: u8, data: &[u8]) -> ServerboundPacket {
            match (packet_id, state) {
                $(
                    ($id, ConnectionState::$state) => {
                        ServerboundPacket::$variant
                        $(
                            ($decoder(data))
                        )?
                    }
                )*
                _ => ServerboundPacket::Unknown(packet_id, data.to_vec()),
            }
        }
    };
}
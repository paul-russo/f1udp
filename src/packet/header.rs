use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;

#[derive(Debug)]
pub struct PacketHeader {
    packet_format: u16,
    // Version of this packet type, all start from 1
    packet_version: u8,
    // Identifier for the packet type
    packet_id: u8,
    // Unique identifier for the session
    session_uid: u64,
    // Session timestamp
    session_time: f32,
    // Identifier for the frame the data was retrieved on
    frame_identifier: u32,
    // Index of player's car in the array
    player_car_index: u8,
}

impl PacketHeader {
    pub fn new<R: BufRead>(reader: &mut R) -> PacketHeader {
        let packet_format = reader.read_u16::<LittleEndian>().unwrap();
        let packet_version = reader.read_u8().unwrap();
        let packet_id = reader.read_u8().unwrap();
        let session_uid = reader.read_u64::<LittleEndian>().unwrap();
        let session_time = reader.read_f32::<LittleEndian>().unwrap();
        let frame_identifier = reader.read_u32::<LittleEndian>().unwrap();
        let player_car_index = reader.read_u8().unwrap();

        PacketHeader {
            packet_format,
            packet_version,
            packet_id,
            session_uid,
            session_time,
            frame_identifier,
            player_car_index,
        }
    }

    pub fn id(&self) -> u8 {
        self.packet_id
    }

    pub fn player_car_index(&self) -> u8 {
        self.player_car_index
    }
}

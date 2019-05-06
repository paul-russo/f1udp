pub mod car_telemetry;
pub mod header;

use car_telemetry::{get_telemetry_array, CarTelemetryData};

use header::PacketHeader;
use std::io::Cursor;

#[derive(Debug)]
pub enum Packet {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry {
        header: PacketHeader,
        data: [CarTelemetryData; 20],
    },
    CarStatus,
}

impl Packet {
    pub fn new(data: &[u8]) -> Packet {
        let mut reader = Cursor::new(data);

        let header = PacketHeader::new(&mut reader);

        match header.id() {
            0 => Packet::Motion,
            1 => Packet::Session,
            2 => Packet::LapData,
            3 => Packet::Event,
            4 => Packet::Participants,
            5 => Packet::CarSetups,
            6 => Packet::CarTelemetry {
                header,
                data: get_telemetry_array(&mut reader),
            },
            7 => Packet::CarStatus,
            _ => panic!(),
        }
    }

    pub fn get_player_telemetry(&self) -> Option<&CarTelemetryData> {
        match self {
            Packet::CarTelemetry { header, data } => {
                Some(&data[header.player_car_index() as usize])
            }
            _ => None,
        }
    }
}

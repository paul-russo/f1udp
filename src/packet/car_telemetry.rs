use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;

#[derive(Debug, Default)]
pub struct CarTelemetryData {
    // Speed of car in kilometres per hour
    speed: u16,
    // Amount of throttle applied (0 to 100)
    throttle: u8,
    // Steering (-100 (full lock left) to 100 (full lock right))
    steer: i8,
    // Amount of brake applied (0 to 100)
    brake: u8,
    // Amount of clutch applied (0 to 100)
    clutch: u8,
    // Gear selected (1-8, N=0, R=-1)
    gear: i8,
    // Engine RPM
    engine_rpm: u16,
    // 0 = off, 1 = on
    drs: u8,
    // Rev lights indicator (percentage)
    rev_lights_percent: u8,
    // Brakes temperature (celsius)
    brakes_temp: [u16; 4],
    // Tires surface temperature (celsius)
    tires_surface_temp: [u16; 4],
    // Tires inner temperature (celsius)
    tires_inner_temp: [u16; 4],
    // Engine temperature (celsius)
    engine_temp: u16,
    // Tires pressure (PSI)
    tires_pressure: [f32; 4],
}

impl CarTelemetryData {
    pub fn new<R: BufRead>(reader: &mut R) -> CarTelemetryData {
        let speed = reader.read_u16::<LittleEndian>().unwrap();
        let throttle = reader.read_u8().unwrap();
        let steer = reader.read_i8().unwrap();
        let brake = reader.read_u8().unwrap();
        let clutch = reader.read_u8().unwrap();
        let gear = reader.read_i8().unwrap();
        let engine_rpm = reader.read_u16::<LittleEndian>().unwrap();
        let drs = reader.read_u8().unwrap();
        let rev_lights_percent = reader.read_u8().unwrap();

        let mut brakes_temp = [0u16; 4];
        reader
            .read_u16_into::<LittleEndian>(&mut brakes_temp)
            .unwrap();

        let mut tires_surface_temp = [0u16; 4];
        reader
            .read_u16_into::<LittleEndian>(&mut tires_surface_temp)
            .unwrap();

        let mut tires_inner_temp = [0u16; 4];
        reader
            .read_u16_into::<LittleEndian>(&mut tires_inner_temp)
            .unwrap();

        let engine_temp = reader.read_u16::<LittleEndian>().unwrap();

        let mut tires_pressure = [0f32; 4];
        reader
            .read_f32_into::<LittleEndian>(&mut tires_pressure)
            .unwrap();

        CarTelemetryData {
            speed,
            throttle,
            steer,
            brake,
            clutch,
            gear,
            engine_rpm,
            drs,
            rev_lights_percent,
            brakes_temp,
            tires_surface_temp,
            tires_inner_temp,
            engine_temp,
            tires_pressure,
        }
    }

    pub fn speed(&self) -> u16 {
        self.speed
    }

    pub fn rev_lights_percent(&self) -> u8 {
        self.rev_lights_percent
    }
}

pub fn get_telemetry_array<R: BufRead>(mut reader: &mut R) -> [CarTelemetryData; 20] {
    let mut data_arr: [CarTelemetryData; 20] = Default::default();

    for el in data_arr.iter_mut() {
        *el = CarTelemetryData::new(&mut reader)
    }

    data_arr
}

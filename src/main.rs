mod packet;

use packet::Packet;
use std::io::ErrorKind;
use std::io::{self, Write};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:20777").expect("Failed to bind socket");
    sock.set_nonblocking(true)
        .expect("Failed to enter non-blocking mode");

    let mut buf = [0u8; 2048];

    loop {
        let result = sock.recv(&mut buf);

        match result {
            // If `recv` was successfull, print the number of bytes received.
            // The received data is stored in `buf`.
            Ok(_num_bytes) => {
                let packet = Packet::new(&buf);

                packet.get_player_telemetry().map(|telemetry| {
                    print!("                \r");
                    print!("{} km/h", telemetry.speed());

                    io::stdout().flush()
                });

                // println!("{:#?}", packet);
            }
            // If we get an error other than "would block", print the error.
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                println!("Something went wrong: {}", err)
            }
            // Do nothing otherwise.
            _ => {}
        }

        thread::sleep(Duration::from_millis(5));
    }
}

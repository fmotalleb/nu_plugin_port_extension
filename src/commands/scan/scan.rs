use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::{Duration, Instant},
};

use nu_protocol::LabeledError;
use std::net::UdpSocket;

use super::{
    scan_config::ScanConfig,
    scan_result::{ScanResult, ScanResultBuilder},
};

pub(super) fn scan(cfg: ScanConfig) -> Result<ScanResult<String>, LabeledError> {
    let mut result = ScanResultBuilder::default();
    result.address(cfg.target_address().to_owned());
    result.port(cfg.target_port().to_owned());
    let now = Instant::now();
    let (is_open, data) = match cfg.udp() {
        true => check_udp(
            cfg.get_socket_addr()?,
            cfg.timeout().to_owned(),
            cfg.send().to_owned().unwrap(),
            cfg.receive_byte_count().to_owned().unwrap(),
        ),
        false => check_tcp(
            cfg.get_socket_addr()?,
            cfg.timeout().to_owned(),
            cfg.send().to_owned(),
            cfg.receive_byte_count().to_owned(),
        ),
    };
    result.elapsed(now.elapsed());
    result.is_open(is_open);
    result.received_data(data);

    Ok(result.build().unwrap())
}
fn check_udp(
    address: SocketAddr,
    timeout: Duration,
    send_data: Vec<u8>,
    receive_byte_count: i64,
) -> (bool, Option<Vec<u8>>) {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if let Err(err) = socket.set_read_timeout(Some(timeout)) {
                eprintln!("Error setting read timeout, {}", err);
                return (false, None);
            }

            if let Err(err) = socket.set_write_timeout(Some(timeout)) {
                eprintln!("Error setting write timeout, {}", err);
                return (false, None);
            }
            if let Err(err) = socket.send_to(&send_data, address) {
                eprintln!("Error sending data, {}", err);
                return (false, None);
            }

            let mut buffer = vec![0; receive_byte_count as usize];
            match socket.peek(&mut buffer) {
                Ok(_) => (true, Some(buffer)),
                Err(err) => {
                    eprintln!("Error receiving data, {}", err);
                    (false, None)
                }
            }
        }
        Err(err) => {
            eprintln!("Error binding UDP socket, {}", err);
            (false, None)
        }
    }
}
fn check_tcp(
    address: SocketAddr,
    timeout: Duration,
    send_data: Option<Vec<u8>>,
    receive_byte_count: Option<i64>,
) -> (bool, Option<Vec<u8>>) {
    match TcpStream::connect_timeout(&address, timeout) {
        Ok(mut stream) => {
            if let Err(err) = stream.set_read_timeout(Some(timeout)) {
                eprintln!("Error setting read timeout, {}", err);
                return (false, None);
            }
            if let Err(err) = stream.set_write_timeout(Some(timeout)) {
                eprintln!("Error setting read timeout, {}", err);
                return (false, None);
            }
            if let Some(data) = send_data {
                if let Err(err) = stream.write_all(&data) {
                    eprintln!("Error writing to socket stream, {}", err);
                    return (false, None);
                }
            }
            if let Some(receive_byte_count) = receive_byte_count {
                // eprintln!("Wait to read the amount of bytes requested");
                let buffer: Result<Vec<u8>, std::io::Error> =
                    stream.bytes().take(receive_byte_count as usize).collect();
                match buffer {
                    Ok(data) => {
                        return (true, Some(data));
                    }
                    Err(err) => {
                        eprintln!("Error reading from socket stream, {}", err);
                        return (false, None);
                    }
                };
            }
            (true, None)
        }
        Err(_) => (false, None),
    }
}

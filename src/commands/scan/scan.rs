use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::{Duration, Instant},
};

use nu_protocol::LabeledError;

use super::{
    scan_config::ScanConfig,
    scan_result::{ScanResult, ScanResultBuilder},
};

pub(super) fn scan(cfg: ScanConfig) -> Result<ScanResult<String>, LabeledError> {
    let mut result = ScanResultBuilder::default();
    result.address(cfg.target_address().to_owned());
    result.port(cfg.target_port().to_owned());
    let now = Instant::now();
    let is_open = check_connection(
        SocketAddr::parse_ascii(cfg.target_address().as_bytes()).map_err(|e| {
            LabeledError::new(format!(
                "cannot parse given address as socket address: {}",
                e
            ))
        })?,
        cfg.timeout().to_owned(),
        cfg.send().to_owned(),
        cfg.receive_byte_count().to_owned(),
    );
    result.elapsed(now.elapsed());
    result.is_open(is_open);

    Ok(result.build().unwrap())
}
fn check_connection(
    address: SocketAddr,
    duration: Duration,
    send_data: Option<Vec<u8>>,
    receive_byte_count: Option<i64>,
) -> bool {
    match TcpStream::connect_timeout(&address, duration) {
        Ok(mut stream) => {
            if let Some(data) = send_data {
                if let Err(err) = stream.write_all(&data) {
                    eprintln!("Error writing to socket stream, {}", err);
                    return false;
                }
            }

            if let Err(err) = stream.set_read_timeout(Some(duration)) {
                eprintln!("Error setting read timeout, {}", err);
                return false;
            }

            if let Some(receive_byte_count) = receive_byte_count {
                // eprintln!("Wait to read the amount of bytes requested");
                let buffer: Result<Vec<u8>, std::io::Error> =
                    stream.bytes().take(receive_byte_count as usize).collect();
                let result = match buffer {
                    Ok(_) => true,
                    Err(err) => {
                        eprintln!("Error reading from socket stream, {}", err);
                        false
                    }
                };
                return result;
            }
            true
        }
        Err(_) => false,
    }
}

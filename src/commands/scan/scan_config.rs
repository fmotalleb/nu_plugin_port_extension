use std::{net::SocketAddr, str::FromStr, time::Duration};

use derive_builder::Builder;
use derive_getters::Getters;
use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Span, Value};

use crate::helpers::FlagHelper;

const DEFAULT_TIMEOUT: Duration = Duration::SECOND;

#[derive(Default, Clone, Builder, Debug, Getters)]
pub(super) struct ScanConfig {
    target_address: String,
    target_port: u16,
    timeout: Duration,
    send: Option<Vec<u8>>,
    receive_byte_count: Option<i64>,
    udp: bool,
}

impl ScanConfig {
    pub fn get_socket_addr(&self) -> Result<SocketAddr, LabeledError> {
        let addr = format!("{}:{}", self.target_address, self.target_port);
        SocketAddr::from_str(&addr).map_err(|e| {
            LabeledError::new(format!(
                "cannot parse given address as socket address: {}",
                e
            ))
        })
    }
}

impl TryFrom<&EvaluatedCall> for ScanConfig {
    type Error = LabeledError;

    fn try_from(call: &EvaluatedCall) -> Result<Self, Self::Error> {
        let mut builder = ScanConfigBuilder::create_empty();

        let addr = call.req::<String>(0).map_err(|e| {
            LabeledError::new(e.to_string()).with_label("failed to get target address", call.head)
        })?;
        builder.target_address(addr);
        let port = call.req::<u16>(1).map_err(|e| {
            LabeledError::new(e.to_string()).with_label("failed to get target port", call.head)
        })?;
        builder.target_port(port);

        let timeout: u64 = match call.get_flag_value("timeout") {
            Some(duration) => duration
                .as_duration()
                .map_err(|e| LabeledError::new(e.to_string()))?
                .try_into()
                .unwrap(),
            None => DEFAULT_TIMEOUT.as_nanos().try_into().unwrap(),
        };
        builder.timeout(Duration::from_nanos(timeout));

        let send_data = match call.get_flag_value("send") {
            Some(Value::String { val, .. }) => Some(val.chars().map(|i| i as u8).collect()),
            Some(Value::Binary { val, .. }) => Some(val),
            _ => None,
        };
        let send_data_exists = send_data.is_some();
        builder.send(send_data);
        let receive_byte_count = match call.get_flag_value("receive-byte-count") {
            Some(Value::Int { val, .. }) => Some(val),
            Some(Value::Filesize { val, .. }) => Some(val.get()),
            _ => None,
        };
        let udp = call.has_flag_or("udp", false);
        builder.udp(udp);
        if udp && (receive_byte_count.is_none() || !send_data_exists) {
            return Err(LabeledError::new(
                "when udp flag is provided receive byte count and send data are mandatory",
            )
            .with_label("udp flag is provided but either one of send or receive-byte-count arguments are missing (both are mandatory)", call.get_flag_span("udp").unwrap_or(Span::unknown())));
        }
        builder.receive_byte_count(receive_byte_count);
        builder.build().map_err(|e| {
            LabeledError::new(format!(
                "Unable to build config from given arguments: {}",
                e.to_string()
            ))
        })
    }
}

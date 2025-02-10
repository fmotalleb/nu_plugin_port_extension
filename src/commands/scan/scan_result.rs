use std::time::Duration;

use derive_builder::Builder;
use nu_protocol::record;

use crate::helpers::AsValue;

#[derive(Default, Clone, Builder, Debug)]
pub(super) struct ScanResult<T: AsValue + ToString> {
    address: T,
    port: u16,
    is_open: bool,
    elapsed: Duration,
}

impl<T: AsValue + ToString> AsValue for ScanResult<T> {
    fn as_value(self, span: nu_protocol::Span) -> nu_protocol::Value {
        record! {
            "address" => self.address.as_value(span),
            "port" => self.port.as_value(span),
            "is_open"=> self.is_open.as_value(span),
            "elapsed" =>  self.elapsed.as_value(span),
        }
        .as_value(span)
    }
}

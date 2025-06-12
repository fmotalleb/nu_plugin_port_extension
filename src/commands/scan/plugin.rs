use crate::PortExtension;
use crate::commands::scan::scan_result::ScanResultBuilder;
use crate::helpers::AsValue;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Span, SyntaxShape};
use std::time::Duration;
use std::vec;

use super::scan;
use super::scan_config::ScanConfig;

#[derive(Default)]
pub struct PortScan {}

impl PortScan {
    pub(crate) fn new() -> PortScan {
        PortScan {}
    }
}

impl PluginCommand for PortScan {
    type Plugin = PortExtension;

    fn name(&self) -> &str {
        "port scan"
    }
    fn signature(&self) -> Signature {
        Signature::build("port scan")
        .required(
        "target IP",
        SyntaxShape::String,
        "target IP address to check for open port",
        )
        .required("port", SyntaxShape::Int, "port to be checked")
        .named(
        "timeout",
        SyntaxShape::Duration,
        "time before giving up the connection. (default: 1 Second)",
        Some('t'),
        )
        .named(
        "send",
        SyntaxShape::OneOf(vec![SyntaxShape::String,SyntaxShape::Binary]),
        "data to send to the target at beginning of the connection",
        Some('s'),
        )
        .named(
        "receive-byte-count",
         SyntaxShape::OneOf(vec![SyntaxShape::Filesize,SyntaxShape::Int]),
         "bytes to receive from the target (possibly after sending the `send` data) to mark the connection as open", 
         Some('b'),
        )
        .switch("udp", "udp scan mod (send and receive-byte-count flags will be mandatory due to how udp works)", Some('u'))
        .category(Category::Network)
    }
    fn description(&self) -> &str {
        "The `port scan` command serves a similar purpose to the `nc -vz {ip} {port}` command,\nIt allows you to detect open ports on a target and provides valuable information about the connection time."
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                example: "port scan 8.8.8.8 53 -t 1sec",
                description: "this will create a Tcp connection to port 53 on 8.8.8.8 (Google's public dns) and return the connection time",
                result: Some(
                    ScanResultBuilder::default()
                        .address("8.8.8.8")
                        .port(53)
                        .is_open(true)
                        .elapsed(Duration::from_millis(27))
                        .received_data(None)
                        .build()
                        .unwrap()
                        .as_value(Span::unknown()),
                ),
            },
            Example {
                example: "port scan 8.8.8.8 54 -t 1sec",
                description: "this will create a Tcp connection to port 54 on 8.8.8.8 (Google's public dns). this will result in an error",
                result: Some(
                    ScanResultBuilder::default()
                        .address("8.8.8.8")
                        .port(54)
                        .is_open(false)
                        .elapsed(Duration::from_secs(1))
                        .received_data(None)
                        .build()
                        .unwrap()
                        .as_value(Span::unknown()),
                ),
            },
            Example {
                example: "port scan 8.8.8.8 53 --udp --receive-byte-count 50 --send ('AAABAAABAAAAAAAAA3d3dwZnb29nbGUDY29tAAABAAEK' | decode base64)",
                description: "send a simple dns request to udp port 53 on 8.8.8.8 (Google's public dns) and return the connection time + received data from dns",
                result: Some(
                    ScanResultBuilder::default()
                        .address("8.8.8.8")
                        .port(53)
                        .is_open(true)
                        .elapsed(Duration::from_millis(27))
                        .received_data(Some(vec![
                            0, 0, 129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 3, 119, 119, 119, 6, 103, 111,
                            111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 192, 12, 0, 1, 0,
                            1, 0, 0, 0, 60, 0, 4, 216, 239, 38, 120,
                        ]))
                        .build()
                        .unwrap()
                        .as_value(Span::unknown()),
                ),
            },
            Example {
                example: "7880..8000 | each { |it| port scan 127.0.0.1 $it -t 1ms } | where is_open",
                description: "This command will scan any port from 7880 to 8000 on localhost and return open ports in range",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        scan::scan(ScanConfig::try_from(call)?)
            .map(|r| PipelineData::Value(r.as_value(call.head), None))
    }
}

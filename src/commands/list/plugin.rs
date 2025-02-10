use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature};

use crate::PortExtension;

use super::{PortListConfig, port_list::collect_socket_info};

#[derive(Default)]
pub struct PortList;

impl PortList {
    pub fn new() -> Self {
        Self {}
    }
}
impl PluginCommand for PortList {
    type Plugin = PortExtension;

    fn name(&self) -> &str {
        "port list-2"
    }

    fn signature(&self) -> Signature {
        Signature::build("port list-2")
            .switch(
                "disable-ipv4",
                "do not fetch ipv6 connections (ipv6 only)",
                Some('6'),
            )
            .switch(
                "disable-ipv6",
                "do not fetch ipv4 connections (ipv4 only)",
                Some('4'),
            )
            .switch(
                "disable-udp",
                "do not fetch UDP connections (TCP only)",
                Some('t'),
            )
            .switch(
                "disable-tcp",
                "do not fetch TCP connections (UDP only)",
                Some('u'),
            )
            .switch(
                "listeners",
                "only listeners (equivalent to state == \"LISTEN\")",
                Some('l'),
            )
            .switch(
                "process-info",
                "loads process info (name, cmd, binary path)",
                Some('p'),
            )
            .category(Category::Network)
    }

    fn description(&self) -> &str {
        "Like netstat this command will return every open connection on the network interface"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let cfg = PortListConfig::try_from(call)?;
        let result = collect_socket_info(cfg)?;
        Ok(PipelineData::Value(result, None))
    }
}

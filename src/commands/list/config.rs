use derive_builder::Builder;
use derive_getters::Getters;
use netstat2::{AddressFamilyFlags, ProtocolFlags};
use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;

use crate::helpers::FlagHelper;

#[derive(Default, Clone, Copy, Builder, Debug, Getters)]
pub struct PortListConfig {
    v4: bool,
    v6: bool,
    tcp: bool,
    udp: bool,
    listeners_only: bool,
    process_info: bool,
}

impl PortListConfig {
    pub fn address_family_flags(self) -> AddressFamilyFlags {
        let mut flags = AddressFamilyFlags::empty();
        if self.v4 {
            flags |= AddressFamilyFlags::IPV4;
        }
        if self.v6 {
            flags |= AddressFamilyFlags::IPV6;
        }
        return flags;
    }
    pub fn protocol_flags(self) -> ProtocolFlags {
        let mut flags = ProtocolFlags::empty();
        if self.tcp {
            flags |= ProtocolFlags::TCP;
        }
        if self.udp {
            flags |= ProtocolFlags::UDP;
        }
        return flags;
    }
}

impl TryFrom<&EvaluatedCall> for PortListConfig {
    type Error = LabeledError;

    fn try_from(value: &EvaluatedCall) -> Result<Self, Self::Error> {
        let mut builder = PortListConfigBuilder::create_empty();
        builder.v4(value.missing_flag_or("disable-ipv4", true));
        builder.v6(value.missing_flag_or("disable-ipv6", true));
        builder.udp(value.missing_flag_or("disable-udp", true));
        builder.tcp(value.missing_flag_or("disable-tcp", true));
        builder.listeners_only(value.has_flag_or("listeners", false));
        return builder
            .build()
            .map_err(|e| LabeledError::new(e.to_string()));
    }
}

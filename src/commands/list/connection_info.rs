use derive_builder::Builder;
use derive_getters::Getters;
use nu_protocol::{Record, Span, record};
use sysinfo::Process;

use crate::helpers::{AsValue, ToStr};

#[derive(Default, Clone, Builder, Debug, Getters)]
pub struct ConnectionInfo {
    pub r#type: String,
    pub ip_version: u32,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: Option<String>,
    pub remote_port: Option<u16>,
    pub state: String,
    pub pid: u32,
    pub process_info: Option<ProcessInfo>,
}
impl Into<Record> for ConnectionInfo {
    fn into(self) -> Record {
        let span = Span::unknown();
        let result = &mut record! {
          "pid"=>self.pid.as_value(span),
          "type" => self.r#type().as_value(span),
          "ip_version" => self.ip_version.as_value(span),
          "local_address" =>self.local_address.as_value(span),
          "local_port" => self.local_port.as_value(span),
          "remote_address" => self.remote_address.as_value(span),
          "remote_port" =>  self.remote_port.as_value(span),
          "state" => self.state.as_value(span),
        };
        if let Some(pf) = self.process_info {
            result.insert("process_name", pf.process_name.as_value(span));
            result.insert("cmd", pf.cmd.as_value(span));
            result.insert("exe_path", pf.exe_path.as_value(span));
            result.insert("process_status", pf.process_status.as_value(span));
            result.insert("process_user", pf.process_user.as_value(span));
            result.insert("process_group", pf.process_group.as_value(span));
            result.insert(
                "process_effective_user",
                pf.process_effective_user.as_value(span),
            );
            result.insert(
                "process_effective_group",
                pf.process_effective_group.as_value(span),
            );
            result.insert(
                "process_environments",
                pf.process_environments.as_value(span),
            );
        }
        return result.to_owned();
    }
}

#[derive(Default, Clone, Builder, Debug)]
pub struct ProcessInfo {
    pub process_name: String,
    pub cmd: Vec<String>,
    pub exe_path: Option<String>,
    pub process_status: String,
    pub process_user: Option<String>,
    pub process_group: Option<String>,
    pub process_effective_user: Option<String>,
    pub process_effective_group: Option<String>,
    pub process_environments: Vec<String>,
}

impl From<&Process> for ProcessInfo {
    fn from(value: &Process) -> Self {
        ProcessInfo {
            cmd: value
                .cmd()
                .to_owned()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            exe_path: value
                .exe()
                .map(|p| p.to_str())
                .unwrap_or(None)
                .map(|s| s.to_string()),
            process_name: value.name().to_string(),
            process_status: value.status().to_string(),
            process_user: value.user_id().map(|id| id.to_string()),
            process_group: value.group_id().map(|id| id.to_string()),
            process_effective_user: value.effective_user_id().map(|id| id.to_string()),
            process_effective_group: value.effective_group_id().map(|id| id.to_string()),
            process_environments: value.environ().iter().map(|i| i.to_string()).collect(),
        }
    }
}

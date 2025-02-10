use std::{collections::HashMap, net::IpAddr, vec};

use netstat2::{
    ProtocolSocketInfo, SocketInfo, TcpSocketInfo, TcpState, UdpSocketInfo, get_sockets_info,
};
use nu_protocol::{LabeledError, Span, Value};
use sysinfo::{Process, System};

use super::{
    PortListConfig,
    connection_info::{ConnectionInfo, ConnectionInfoBuilder, ProcessInfo},
};

pub(super) fn collect_socket_info(cfg: PortListConfig) -> Result<Value, LabeledError> {
    let system = System::new_all();
    let mut process_list: HashMap<u32, &Process> = HashMap::new();
    if *cfg.process_info() {
        process_list = system
            .processes()
            .iter()
            .map(|(pid, process)| (pid.as_u32(), process))
            .collect();
    }
    let connections = sockets(&process_list, cfg)?;
    let result = connections
        .iter()
        .map(|f| Value::record(f.to_owned().into(), Span::unknown()));

    Ok(Value::list(result.collect(), Span::unknown()))
}

fn sockets(
    processes: &HashMap<u32, &Process>,
    cfg: PortListConfig,
) -> Result<Vec<ConnectionInfo>, LabeledError> {
    let af = cfg.address_family_flags();
    let pf = cfg.protocol_flags();
    let sockets_info = get_sockets_info(af, pf).map_err(|e| {
        LabeledError::new(e.to_string()).with_code("netstat2::get_sockets_info::error")
    })?;
    let connections = sockets_info
        .iter()
        .map(|i| socket2connection_info(cfg, processes, i))
        .flatten();
    let result = connections
        .map(|f| f.build().map_err(|e| LabeledError::new(e.to_string())))
        .collect();
    let (answer, errs) = split_results(result);
    if let Some(err) = errs.first() {
        return Err(err.to_owned());
    }
    Ok(answer)
}

fn split_results<A, E>(input: Vec<Result<A, E>>) -> (Vec<A>, Vec<E>) {
    let mut ok_vec = vec![];
    let mut err_vec = vec![];
    for item in input {
        match item {
            Ok(i) => ok_vec.push(i),
            Err(e) => err_vec.push(e),
        }
    }
    (ok_vec, err_vec)
}

fn socket2connection_info(
    cfg: PortListConfig,
    processes: &HashMap<u32, &Process>,
    si: &SocketInfo,
) -> Vec<ConnectionInfoBuilder> {
    match si.to_owned().protocol_socket_info {
        ProtocolSocketInfo::Tcp(tcp_socket_info)
            if *cfg.tcp()
                && (!cfg.listeners_only() || tcp_socket_info.state == TcpState::Listen) =>
        {
            tcp2connection_info(processes, si, tcp_socket_info)
        }
        ProtocolSocketInfo::Udp(udp_socket_info) if *cfg.udp() => {
            udp2connection_info(processes, si, udp_socket_info)
        }
        _ => vec![],
    }
}
fn tcp2connection_info(
    processes: &HashMap<u32, &Process>,
    si: &SocketInfo,
    tsi: TcpSocketInfo,
) -> Vec<ConnectionInfoBuilder> {
    si.associated_pids
        .iter()
        .map(|pid| {
            ConnectionInfoBuilder::default()
                .pid(pid.to_owned())
                .r#type("tcp".to_string())
                .ip_version(get_ip_version(tsi.local_addr))
                .local_address(tsi.local_addr.to_string())
                .local_port(tsi.local_port)
                .remote_address(Some(tsi.remote_addr.to_string()))
                .remote_port(Some(tsi.remote_port))
                .state(tsi.state.to_string())
                .process_info(
                    processes
                        .get(pid)
                        .map(|process| ProcessInfo::from(process.to_owned()))
                        .map_or(None, |f| Some(f)),
                )
                .to_owned()
        })
        .collect()
}
fn udp2connection_info(
    processes: &HashMap<u32, &Process>,
    si: &SocketInfo,
    usi: UdpSocketInfo,
) -> Vec<ConnectionInfoBuilder> {
    si.associated_pids
        .iter()
        .map(|pid| {
            ConnectionInfoBuilder::default()
                .pid(pid.to_owned())
                .r#type("udp".to_string())
                .ip_version(get_ip_version(usi.local_addr))
                .local_address(usi.local_addr.to_string())
                .local_port(usi.local_port)
                .remote_address(None)
                .remote_port(None)
                .state("LISTEN".to_string())
                .process_info(
                    processes
                        .get(pid)
                        .map(|process| ProcessInfo::from(process.to_owned())),
                )
                .to_owned()
        })
        .collect()
}
fn get_ip_version(addr: IpAddr) -> u32 {
    match addr {
        IpAddr::V4(_) => 4,
        IpAddr::V6(_) => 6,
    }
}

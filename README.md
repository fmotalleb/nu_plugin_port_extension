# Nushell Port Extension

This Nushell plugin introduces two subcommands under `port`:

- `port list`: Lists all open connections, similar to `netstat`.
- `port scan`: Scans a target IP and port to check if it is open.

## Installation

To install the plugin, follow these steps:
- remove old plugins:
```bash
plugin rm port_list
plugin rm port_scan
```

- using [nupm](https://github.com/nushell/nupm)

```bash
git clone https://github.com/FMotalleb/nu_plugin_port_extension.git
nupm install --path nu_plugin_port_extension -f
```

- or compile manually

```bash
git clone https://github.com/FMotalleb/nu_plugin_port_extension.git
cd nu_plugin_port_extension
cargo build -r
plugin add target/release/nu_plugin_port_extension
```

- or using cargo

```bash
cargo install nu_plugin_port_extension
# or cargo install --git https://github.com/FMotalleb/nu_plugin_port_extension.git (sometimes I am unable to update my package due to sanctions)
plugin add ~/.cargo/bin/nu_plugin_port_extension
```

## Commands

### `port list`

The `port list` command displays every open connection on the network interface.

#### Usage

```sh
> port list {flags}
```

#### Flags

- `-h, --help`                 : Display the help message for this command
- `-6, --disable-ipv4`         : Do not fetch IPv4 connections (IPv6 only)
- `-4, --disable-ipv6`         : Do not fetch IPv6 connections (IPv4 only)
- `-t, --disable-udp`          : Do not fetch UDP connections (TCP only)
- `-u, --disable-tcp`          : Do not fetch TCP connections (UDP only)
- `-l, --listeners`            : Only show listeners (equivalent to state == "LISTEN")
- `-p, --process-info`         : Include process info (name, cmd, binary path)

#### Examples
```bash
port list -p | take 1
```

|pid|type|ip_version|local_address|local_port|remote_address|remote_port|state|process_name|cmd|exe_path|process_status|process_user|process_group|process_effective_user|process_effective_group|process_environments|
|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|
|11536|tcp|4|127.0.0.1|1093|127.0.0.1|1108|ESTABLISHED|steam.exe|[C:\Program Files (x86)\Steam\steam.exe, -silent]|C:\Program Files (x86)\Steam\steam.exe|Runnable|S-1-5-21-1866364434-2240987276-2714941485-1001||||[]|



### `port scan`

The `port scan` command detects open ports on a target IP, similar to `nc -vz {ip} {port}`.
the limitation is it only supports TCP ports for now

#### Usage

```sh
> port scan {flags} <target IP> <port>
```

#### Flags

- `-h, --help`                : Display the help message for this command
- `-t, --timeout <duration>`  : Set timeout before giving up the connection (default: 60s)
- `-s, --send <string>`       : Data to send to the target at the start of the connection
- `-b, --receive-byte-count <int>` : Number of bytes to receive before marking connection as open

#### Parameters

- `target IP <string>` : Target IP address
- `port <int>`         : Port to scan

#### Examples

Check if port 53 (TCP) is open on Google's public DNS:

```sh
> port scan 8.8.8.8 53 -t 1sec
```

Output:

```
╭─────────┬─────────╮
│ address │ 8.8.8.8 │
│ port    │ 53      │
│ is_open │ true    │
│ elapsed │ 40ms    │
╰─────────┴─────────╯
```

Scan a range of ports on localhost and filter for open ports:

```sh
> 7880..8000 | each { |it| port scan 127.0.0.1 $it -t 1ms } | where result == Open
```

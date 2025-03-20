# 🔌 nu_plugin_port_extension  

A [Nushell](https://www.nushell.sh/) plugin for inspecting open ports and scanning network services. It introduces two subcommands:  

- **`port list`**: Lists all open network connections, similar to `netstat`.  
- **`port scan`**: Scans a target IP and port to determine if it is open.  

---

## ✨ Features  

### **`port list`** – List Active Connections  
The `port list` command retrieves all open connections on the network interface. It supports filtering by protocol, IP version, and listening state.  

#### 📌 Usage  
```bash
port list {flags}
```  

#### ⚙️ Available Flags  
- `-h, --help`                 → Show help message.  
- `-6, --disable-ipv4`         → Exclude IPv4 connections (only show IPv6).  
- `-4, --disable-ipv6`         → Exclude IPv6 connections (only show IPv4).  
- `-t, --disable-udp`          → Exclude UDP connections (only show TCP).  
- `-u, --disable-tcp`          → Exclude TCP connections (only show UDP).  
- `-l, --listeners`            → Show only listening connections (`state == "LISTEN"`).  
- `-p, --process-info`         → Include process details (name, command, binary path).  

#### 🔍 Example: Show Active Processes  
```bash
port list -p | take 1
```  

#### 📊 Example Output  
|pid  |type|ip_version|local_address|local_port|remote_address|remote_port|state      |process_name|cmd                                               |exe_path                              |  
|-----|----|----------|-------------|----------|--------------|----------|-----------|------------|--------------------------------------------------|--------------------------------------|  
|11536|tcp |4         |127.0.0.1    |1093      |127.0.0.1     |1108      |ESTABLISHED|steam.exe   |[C:\Program Files (x86)\Steam\steam.exe, -silent]|C:\Program Files (x86)\Steam\steam.exe|  

---

### **`port scan`** – Scan Open Ports  
The `port scan` command checks if a specific port is open on a target IP, similar to `nc -vz {ip} {port}`.  

> **⚠️ Note:** Only **TCP** scanning is supported at the moment.  

#### 📌 Usage  
```bash
port scan {flags} <target IP> <port>
```  

#### ⚙️ Available Flags  
- `-h, --help`                 → Show help message.  
- `-t, --timeout <duration>`   → Set timeout before giving up (default: 60s).  
- `-s, --send <string>`        → Send data to the target upon connection.  
- `-b, --receive-byte-count <int>` → Number of bytes to receive before confirming the connection is open.  

#### 🎯 Parameters  
- **`target IP`** *(string)* – The IP address to scan.  
- **`port`** *(integer)* – The port number to check.  

#### 🔍 Example: Check if Google's Public DNS (8.8.8.8) has Port 53 Open  
```bash
port scan 8.8.8.8 53 -t 1sec
```  

#### 📊 Example Output  
```
╭─────────┬─────────╮  
│ address │ 8.8.8.8 │  
│ port    │ 53      │  
│ is_open │ true    │  
│ elapsed │ 40ms    │  
╰─────────┴─────────╯  
```  

#### 🔄 Example: Scan a Range of Ports on `127.0.0.1` and Filter Open Ports  
```bash
7880..8000 | each { |it| port scan 127.0.0.1 $it -t 1ms } | where result == Open
```  

---

## 🔧 Installation  

### 🚀 Recommended: Using [nupm](https://github.com/nushell/nupm)  
This method automatically handles dependencies and features.  
```bash
git clone https://github.com/FMotalleb/nu_plugin_port_extension.git  
nupm install --path nu_plugin_port_extension -f  
```  

### 🛠️ Manual Compilation  
```bash
git clone https://github.com/FMotalleb/nu_plugin_port_extension.git  
cd nu_plugin_port_extension  
cargo build -r  
plugin add target/release/nu_plugin_port_extension  
```  

### 📦 Install via Cargo (using git)  
```bash
cargo install --git https://github.com/FMotalleb/nu_plugin_port_extension.git  
plugin add ~/.cargo/bin/nu_plugin_port_extension  
```  

### 📦 Install via Cargo (crates.io) _Not Recommended_  
> *Since I live in Iran and crates.io often restricts package updates, the version there might be outdated.*  
```bash
cargo install nu_plugin_port_extension  
plugin add ~/.cargo/bin/nu_plugin_port_extension  
```  

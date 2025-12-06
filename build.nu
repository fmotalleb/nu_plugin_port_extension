use std log

def addcap [binpath: path] {
    if ($nu.os-info.name == "linux") {
        print $"(ansi red)Warning(ansi reset): To access the PIDs of applications running as root or other users, you need to add the following capabilities to the plugin binary: net_admin, sys_ptrace, dac_read_search."
        print $"(ansi red)sudo setcap 'cap_net_admin,cap_sys_ptrace,cap_dac_read_search=ep' '($binpath)'(ansi reset)"
        print "Do you want to run this command with sudo? [y/N]"
        
        match (input listen --types [key]) {
            {code: $key} if ($key | str downcase) == "y" => {
                let caps = (['cap_net_admin', 'cap_sys_ptrace', 'cap_dac_read_search'] | input list --multi "Choose which specific capabilities you want to add" | str join ",")
                let cmd = $"setcap '($caps)=ep' '($binpath)'"
                print $"Executing `($cmd)` with sudo."
                sh -c $"sudo ($cmd)"
            }
            _ => {
                print "If you prefer to add the capabilities manually, use the following command:"
                print $"sudo setcap 'cap_net_admin,cap_sys_ptrace,cap_dac_read_search=ep' '($binpath)'"
            }
        }
    } else if ($nu.os-info.name == "windows") {
        print $"(ansi red)For Windows Users(ansi reset): The plugin is installed successfully but keep in mind in order to fetch all executables information you must run nushell as administrator"
    }
}

def main [package_file: path] {
    
    let repo_root = $package_file | path dirname
    let install_root = $env.NUPM_HOME | path join "plugins"

    let name = open ($repo_root | path join "Cargo.toml") | get package.name
    let cmd = $"cargo install --path \"($repo_root)\" --root \"($install_root)\""

    log info $"building plugin using: (ansi blue)($cmd)(ansi reset)"
    nu -c $cmd
    let ext: string = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    let bin_path = $"($install_root | path join "bin" $name)($ext)"
    plugin add $bin_path
    addcap $bin_path

    log info "do not forget to restart Nushell for the plugin to be fully available!"
}

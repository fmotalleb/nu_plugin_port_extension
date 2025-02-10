mod commands;
mod helpers;

use commands::{list::PortList, scan::PortScan};

use nu_plugin::PluginCommand;
pub struct PortExtension;

impl nu_plugin::Plugin for PortExtension {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(PortList::new()), Box::new(PortScan::new())]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}

fn main() {
    nu_plugin::serve_plugin(&mut PortExtension {}, nu_plugin::MsgPackSerializer {})
}

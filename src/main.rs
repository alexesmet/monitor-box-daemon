use serialport;
use std::env;
use std::thread;
use std::time::Duration;
use sysinfo::{self, ProcessorExt, SystemExt};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} [port]", args[0]);
        if let Ok(ports) = serialport::available_ports() {
            println!("List of available ports: ");
            for port in &ports {
                println!(" - {} ", port.port_name);
            }
        }
    } else {
        let port_address = &args[1];
        let port_result = serialport::open(&port_address);
        match port_result {
            Err(serialport::Error {kind, description}) => println!("{:?} error while opening port: {}", kind, description),
            Ok(mut port) => {
                let mut sys = sysinfo::System::new_all();
                loop {
                    sys.refresh_all();
                    match write_sys_to_port(&mut port, &sys) {
                        Ok(()) => {},
                        Err(serialport::Error {kind, description}) => 
                            println!("{:?} error while writing to port: {}", kind, description)
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
}

fn write_sys_to_port(port: &mut Box<dyn serialport::SerialPort>, sys: &sysinfo::System) -> Result<(), serialport::Error> {
    // write proc
    port.write(b"p")?;
    let usage =  format!("{:.0}", sys.get_global_processor_info().get_cpu_usage());
    port.write(&usage.as_bytes())?;
    port.write(b"\0")?;
    // write ram
    port.write(b"m")?;
    let usage =  format!("{:.0}", sys.get_used_memory() * 100 / sys.get_total_memory() );
    port.write(&usage.as_bytes())?;
    port.write(b"\0")?;
    return Ok(());
}



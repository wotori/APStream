use rusb::{Context, UsbContext};
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("USB error: {0}")]
    Usb(#[from] rusb::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

type Result<T> = std::result::Result<T, MyError>;

fn main() -> Result<()> {
    let mut file = File::create("usb_traffic.log")?;
    loop {
        let ports = get_ports_info();
        for port in ports {
            if port.status == "Device connected" {
                if let Err(e) = process_port(&port, &mut file) {
                    eprintln!("Error processing port {}: {}", port.receptacle, e);
                }
            }
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn get_ports_info() -> Vec<PortInfo> {
    let output = Command::new("system_profiler")
        .arg("SPThunderboltDataType")
        .output()
        .expect("Failed to execute system_profiler");

    parse_ports_info(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_ports_info(data: String) -> Vec<PortInfo> {
    let mut ports = Vec::new();
    let mut current_receptacle = String::new();

    for line in data.lines() {
        if line.contains("Receptacle") {
            current_receptacle = line.split(':').nth(1).unwrap_or("").trim().to_string();
        }
        if line.contains("Status") {
            let status = if line.contains("No device connected") {
                "No device connected"
            } else {
                "Device connected"
            };
            let port = PortInfo {
                receptacle: current_receptacle.clone(),
                status: status.to_string(),
            };
            ports.push(port);
        }
    }
    ports
}

#[derive(Debug)]
struct PortInfo {
    receptacle: String,
    status: String,
}
fn process_port(port: &PortInfo, file: &mut File) -> Result<()> {
    let context = Context::new()?;
    let devices = context.devices()?;
    for device in devices.iter() {
        let device_desc = device.device_descriptor()?;
        let handle = device.open()?;
        println!(
            "Device {:04x}:{:04x} connected to port {}",
            device_desc.vendor_id(),
            device_desc.product_id(),
            port.receptacle
        );

        let endpoint_address = 0x81;
        let interface_number = 0;

        handle.claim_interface(interface_number)?;

        let mut buffer = [0u8; 64];

        loop {
            match handle.read_bulk(endpoint_address, &mut buffer, Duration::from_secs(1)) {
                Ok(size) => {
                    file.write_all(&buffer[..size])?;
                    file.flush()?;
                }
                Err(rusb::Error::Timeout) => continue,
                Err(e) => {
                    eprintln!("Error reading from device: {}", e);
                    break;
                }
            }
        }

        handle.release_interface(interface_number)?;
    }

    Ok(())
}

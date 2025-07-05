use clap::{Arg, Command};
use comfy_table::Table;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Read}, path::PathBuf,
};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    name: String,
    macs: Vec<String>,
}

fn get_nmap_output() -> Result<IndexMap<String, String>, Box<dyn std::error::Error>> {
    let mut xml_input = String::new();
    io::stdin().read_to_string(&mut xml_input)?;
    let parser = EventReader::from_str(&xml_input);
    let mut ip_mac_map: IndexMap<String, String> = IndexMap::new();
    let mut current_ip = None;
    for e in parser {
        match e? {
            XmlEvent::StartElement {
                name, attributes, ..
            } if name.local_name == "address" => {
                let mut addr = None;
                let mut addrtype = None;

                for attr in attributes {
                    match attr.name.local_name.as_str() {
                        "addr" => addr = Some(attr.value),
                        "addrtype" => addrtype = Some(attr.value),
                        _ => {}
                    }
                }
                if let (Some(addr), Some(addrtype)) = (addr, addrtype) {
                    if addrtype == "ipv4" {
                        current_ip = Some(addr);
                    } else if addrtype == "mac" {
                        if let Some(ip) = current_ip.take() {
                            ip_mac_map.insert(ip, addr);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return Ok(ip_mac_map);
}

fn get_devices() -> Result<Vec<Device>, Box<dyn std::error::Error>> {
    let matches = Command::new("Nmap MAC Matcher")
        .arg(
            Arg::new("devices")
                .short('d')
                .long("devices")
                .value_name("FILE")
                .help("Path to JSON file containing devices definitions")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .get_matches();
    
    let devices_file = matches.get_one::<PathBuf>("devices").unwrap();
    let json_content = fs::read_to_string(devices_file)?;
    let devices: Vec<Device> = serde_json::from_str(&json_content)?;
    Ok(devices)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nmap_output: IndexMap<String, String> = get_nmap_output()?;
    let devices: Vec<Device> = get_devices()?;

    let mac_to_device: IndexMap<String, String> = devices
        .iter()
        .flat_map(|device| {
            device
                .macs
                .iter()
                .map(move |mac| (mac.to_uppercase(), device.name.clone()))
        })
        .collect();

    let mut table = Table::new();
    table.set_header(vec!["IP", "MAC", "Device"]);
    for (ip, mac) in nmap_output {
        let mac_uppercase = mac.to_uppercase();
        let device_name = mac_to_device
            .get(&mac_uppercase)
            .map(|name| name.as_str())
            .unwrap_or("Unknown");

        table.add_row(vec![ip, mac_uppercase, device_name.to_string()]);
    }

    println!("{table}");

    Ok(())
}

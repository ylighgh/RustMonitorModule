use std::fs::File;
use std::io;
use std::io::prelude::*;
use serde_json::json;

struct HostName {
    hostname: String,
}

trait Util {
    fn to_json(&self) -> String;
}

fn get_host_name() -> Result<HostName, io::Error> {
    let mut file = File::open("/etc/hostname")
        .expect("File not found!");
    let mut result: String = String::new();
    file.read_to_string(&mut result)
        .expect("Error while reading file");
    let vv: Vec<&str> = result.split(" ").collect();
    let host_name = HostName {
        hostname: vv.get(0).unwrap().parse().unwrap(),
    };
    Ok(host_name)
}

impl HostName {
    fn new() -> HostName {
        get_host_name().unwrap()
    }
}

impl Util for HostName {
    fn to_json(&self) -> String {
        let value = json!({
            "hostname":self.hostname,
        }).to_string();
        value
    }
}

fn main() {
    println!("{}", HostName::new().to_json());
}

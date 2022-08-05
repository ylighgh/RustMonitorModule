use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct DnsServer {
    dns1: String,
    dns2: String,
}

impl DnsServer {
    fn new() -> Result<DnsServer, io::Error> {
        let mut result: Vec<String> = Vec::new();

        let file = File::open("/etc/resolv.conf").expect("open failed");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if line.as_ref().unwrap().starts_with("nameserver") {
                for l in line.as_ref().unwrap().split(" ") {
                    if !l.starts_with("nameserver") {
                        result.push(l.parse().unwrap());
                    }
                };
            }
        };
        let dns_server = DnsServer {
            dns1: result.get(0).unwrap().parse().unwrap(),
            dns2: result.get(1).unwrap().parse().unwrap(),
        };
        Ok(dns_server)
    }
}

fn main() {
    let dns_server = DnsServer::new().unwrap();
    println!("DNS1:{}\nDNS2:{}", dns_server.dns1, dns_server.dns2);
}

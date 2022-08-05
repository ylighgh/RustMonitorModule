use serde_json::json;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct LoadStat {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

trait Util {
    fn to_json(&self) -> String;
}


impl LoadStat {
    fn new() -> Result<LoadStat, io::Error> {
        let mut file = File::open("/proc/loadavg").expect("File not found!");
        let mut result: String = String::new();
        file.read_to_string(&mut result)
            .expect("Error while reading file");
        let vv: Vec<&str> = result.split(" ").collect();
        let load_stat = LoadStat {
            one: vv.get(0).unwrap().parse::<f64>().unwrap(),
            five: vv.get(1).unwrap().parse::<f64>().unwrap(),
            fifteen: vv.get(2).unwrap().parse::<f64>().unwrap(),
        };
        Ok(load_stat)
    }
}

impl Util for LoadStat {
    fn to_json(&self) -> String {
        let value = json!({
            "one":self.one,
            "five":self.five,
            "fifteen":self.fifteen,
        })
            .to_string();
        return value;
    }
}

//程序入口
fn main() {
    println!("{}", LoadStat::new().unwrap().to_json());
}

use std::time::Duration;
use std::thread::sleep;

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::fs::File;

use http_prober::prober_logic::*;

#[derive(Serialize, Deserialize)]
struct Config {
  period: u64,
  ip_and_port: String,
  uri: String,
  threshold: u16
}

fn main() {
    // TODO: Add a file with the OCI spec

    let mut f = File::open("configuration.json").unwrap();
    let mut json_config = String::new();
    f.read_to_string(&mut json_config);
    let config: Config = serde_json::from_str(&json_config).expect("JSON was not well-formatted");
    let mut counter = 0;

    loop { 
      if let Ok(msg) = probe(&config.ip_and_port, &config.uri) {
    
          match check_response(&msg) {
            
            Success => {
              counter+=1;
              if counter >= config.threshold {
                println!("Status: Healthy");
                counter = 0;
              }
            },
            Failed => {
              counter = 0;
              println!("Status: Unhealthy");
            }
          
          }
      }
      sleep(Duration::new(config.period,0));
    }

}


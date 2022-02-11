pub mod prober_logic {
    use std::io::prelude::*;
    use std::net::TcpStream;
    use std::time::Duration;

    use regex::Regex;

    #[derive(Debug)]
    pub enum ProbeResult {
      
      Success,
      Failed

    }

    pub fn probe(ip_and_port: &str, uri: &str) -> std::io::Result<String>{

        if let Ok(mut stream) = TcpStream::connect(ip_and_port) {
          println!("Connected to the server!");
          stream.write(format!("GET {} HTTP/1.1\n
                         Host: www.google.com\n
                         Connection: close", uri).as_bytes())?;
          stream.flush()?;

          match stream.set_read_timeout(Some(Duration::new(5,0)))
          {

              Ok(_) => {

                let mut buffer = [0; 1000];
                let n = stream.read(&mut buffer[..])?;
                let res: String = String::from_utf8_lossy(&buffer[..n]).to_string();
                return Ok(res);

              },
              Err(error) => {
                print!("An error was found: {:?}", error);
              }

          }

        } else {
          println!("Couldn't connect to server...");
        }

        Ok(String::from("no response"))

    }

    pub fn check_response(msg: &str) -> ProbeResult {

      let re = Regex::new(r"HTTP/1.1 200 OK").unwrap();
      if re.is_match(msg) {
        ProbeResult::Success
      } else {
        ProbeResult::Failed
      }

    }

}

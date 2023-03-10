use std::{
    mem::size_of,
    io::{
        Read,
        Write,
        BufReader, BufRead,
    },
    net::{
        Ipv4Addr,
        TcpStream
    },
};

pub struct Communicator {
    ip_address: Ipv4Addr,
    sock: u16
}

impl Communicator {
    
    pub fn new(ip: (u8, u8, u8, u8), sock: u16) -> Self {
        Self {
            ip_address: Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3),
            sock
        }
    }

    pub fn get_current_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        if let Ok(mut sock) = TcpStream::connect((self.ip_address, self.sock)) {
            const CURRENT_PRICE_CMD: &str = "get_current_price";
            let mut resp = String::new();
            sock.write(CURRENT_PRICE_CMD.as_bytes())?;
            let mut buff_sock = BufReader::new(sock);
            buff_sock.read_line(&mut resp);
            resp.clear();
            match buff_sock.read_line(&mut resp) {
                Ok(_) => {
                    println!("Received {}", resp);
                    match resp.trim().parse() {
                        Ok(price) => return Ok(price),
                        Err(e) => {
                            println!("{}", e);
                            return Ok(-1.);
                        }
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    return Ok(-1.);
                }
            }
        } else {
            return Ok(-1.);
        }
    }
}

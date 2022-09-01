use std::net::{UdpSocket};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub struct UDPSktTools {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub online: bool,
    pub skt: Result<UdpSocket, bool>,
}

impl UDPSktTools {
    pub fn new(str_addr: Option<String>) -> Self {
        match str_addr {
            None => {
                Self {
                    name: Default::default(),
                    ip: "0.0.0.0".to_string(),
                    port: 8080,
                    online: false,
                    skt: match UdpSocket::bind("0.0.0.0:8080") {
                        Ok(s) => Ok(s),
                        Err(_) => Err(true)
                    },
                }
            }
            Some(sa) => {
                let a: Vec<&str> = sa.split(":").collect();
                let ip: &str = a[0];
                let port: u16 = a[1].parse().expect("端口非法");
                Self {
                    name: Default::default(),
                    ip: ip.clone().to_string(),
                    port: port.clone(),
                    online: false,
                    skt: match UdpSocket::bind(sa.clone()) {
                        Ok(s) => Ok(s),
                        Err(_) => Err(true)
                    },
                }
            }
        }
    }

    pub fn set_online(&mut self, active: bool) {
        self.online = active;
    }

    pub fn get_online(&self) -> bool {
        self.online
    }

    pub fn restart_skt(&mut self) -> bool {
        let sa = format!("{}:{}", self.ip.clone(), self.port.clone());
        self.skt = match UdpSocket::bind(sa) {
            Ok(s) => {
                self.online = true;
                Ok(s)
            }
            Err(_) => Err(true)
        };
        match self.skt {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn gen_sha256(&self,hashme: &str) -> String {
        let mut sh = Sha256::new();
        sh.input_str(hashme);
        sh.result_str()
    }
}
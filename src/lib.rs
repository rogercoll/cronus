use std::net::TcpListener;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    pub filename: String,
}

pub struct Server {
    pub port: String,
    pub banner: String,
    password: String,
    listener: TcpListener,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 2 {
            return Err("Not enought arguments!");
        }
        let filename = args[1].clone();
        Ok(Config {filename})
    }
}

impl Server {
    pub fn new(port: &str, password: &str, banner: &str) -> Result<Server, &'static str> {
        let port = port.to_owned();
        let password = password.to_owned();
        let banner = banner.to_owned();
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();    
        Ok(Server {port, password, banner, listener})
    }
    pub fn run(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            println!("Connection received");
            let mut stream = stream.unwrap();
            let response = filename;
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        };
        Ok(())
    }
}




use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
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
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let reqPass = String::from_utf8_lossy(&buffer[..]);
            let corrPass = &self.password;
            match reqPass {
                corrPass => {
                    println!("Correct password"); 
                    let response = filename;
                    stream.write(response.as_bytes()).unwrap();
                    self.sendFile(stream);
                },
                _ => println!("Wrong password"),
            }
        };
        Ok(())
    }
    fn sendFile(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut file = File::open("./hello.txt").unwrap();
        let mut bytes_count: i32 = 0;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let size = buffer.len();
        println!("{}", size);
        println!("{:?}", &buffer);
        stream.write(&buffer).unwrap();
        stream.flush().unwrap();
        Ok(())
    }
}



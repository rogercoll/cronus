use std::io::prelude::*;
use cronus::Config;
use cronus::Server;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{}", config.filename);
    let server = Server::new("hello.txt", "pass123", "Hellow fool").unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{}", server.banner);
    server.run(&config.filename);
}

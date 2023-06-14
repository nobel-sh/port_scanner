use clap::{Parser};
use scanner::Scanner;
use std::collections::HashMap;

mod scanner;
mod args;

use args::Args;

fn get_service_name()->HashMap<u16, String>{
    let file = std::fs::read_to_string("./tcp_service.txt").expect("File not found");

    let mut services = file.split('\n');

    let mut service_map = HashMap::new();
    loop{
        let line = services.next();
        
        if line.is_none() || !line.unwrap().contains(", "){
            break;
        }
        let mut splited = line.unwrap().split(", ");
        let port = splited.next().unwrap().parse::<u16>().unwrap();
        let service = splited.next().unwrap();
        service_map.insert(port, String::from(service));
    }
    service_map

}

fn main() {    
    let arguments = Args::parse();
    let addrs = arguments.get_ip();
    let ports = arguments.get_ports();

    let mut scanner;
    if let Some(threads) = arguments.get_threads(){
        
        scanner = Scanner::new_with_workers(addrs, ports, 3, threads);
    }else{
        scanner = Scanner::new(addrs, ports, 3);
    }

    let starting_time = std::time::Instant::now();
    println!(" [***] Starting Scan\n");
    let open_ports = scanner.scan();

    let service_map = get_service_name();

    for  socket in open_ports{
        println!(" [*] IP : {}",socket.0);
        println!(" [*] ports : ",);
        if socket.1.is_empty(){
            println!("      [-] No Services found");
            continue;
        }
        for port in socket.1{
            println!("      [+] {port} : {}",service_map.get(&port).unwrap());
        }
        println!(" ");
    }
    let time_taken = std::time::Instant::now()-starting_time;
    println!(" [*] Time Taken : {}",time_taken.as_secs().to_string());
    println!(" [***] Scan Finished\n");
    
}




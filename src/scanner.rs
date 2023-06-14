use std::net::{SocketAddr,TcpStream,IpAddr};
use std::time::Duration;
use std::sync::{Mutex,Arc};
use std::collections::HashMap;

pub struct Scanner{
    addrs:Vec<IpAddr>,
    ports:Vec<u16>,
    timeout: u16,
    workers:u16,
}

impl Scanner{
    pub fn new(addrs:Vec<IpAddr>,ports:Vec<u16>,timeout:u16)->Self{
        Self{addrs,ports,timeout,workers:5}
    }
    
    pub fn new_with_workers(addrs:Vec<IpAddr>,ports:Vec<u16>,timeout:u16,workers:u16)->Self{
        Self{addrs,ports,timeout,workers}
    }

    pub fn scan(&mut self)-> HashMap<&IpAddr, Vec<u16>>{
        let mut ip_port_map = HashMap::new();

        for addr in &self.addrs {
            let open_ports: Arc<Mutex<Vec<u16>>> = Arc::default();
            let mut port_clone = self.ports.clone();
        
            while !port_clone.is_empty(){
                
                let mut threads = vec![];
                for _ in 0..self.workers{
                    let _port = port_clone.pop();
                    let port = match _port{
                        Some(p)=> p,
                        None => break,
                    };

                    let socket_addr = SocketAddr::new(*addr, port);
                    let timeout = self.timeout;
                    
                    let _open_ports = open_ports.clone();
                    let handle = std::thread::spawn({
                        move || if let Ok(_) = TcpStream::connect_timeout(
                            &socket_addr,
                            Duration::from_secs(timeout as u64),
                        ){
                            let mut open_ports_lock = _open_ports.lock().unwrap();
                            open_ports_lock.push(port);
                        }
                    });
                    threads.push(handle);
                }
                for handle in threads{
                    handle.join().unwrap();
                }   
            }
            ip_port_map.insert(addr, Arc::try_unwrap(open_ports).unwrap().into_inner().unwrap());
        }

        ip_port_map
    }
    
}

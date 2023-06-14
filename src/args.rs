use std::net::IpAddr;
use clap::Parser;


#[derive(Parser,Debug)]
pub struct Args{
    /// ip address of the machine to scan
    #[arg(short,long)]
    ip:String,
    /// ports to scan
    #[arg(long="port",short,value_delimiter=',')]
    ports:Vec<String>,
    /// number of threads to spawn
    #[arg(long,short)]
    threads:Option<u16>
}

impl Args{
    pub fn get_ports(&self)->Vec<u16>{
        let mut ports = vec![];
        for port in &self.ports{
            if port.contains('-'){
                let splited:Vec<u16>= port.split('-').map(|p| p.parse::<u16>().expect("Invalid port given")).collect();
                for i in splited[0]..=splited[1]{
                    ports.push(i);
                }
            }else{
                ports.push(port.parse::<u16>().expect("Invalid port given"));
            }
        }
    
        //remove duplicates
        ports.sort();
        ports.dedup();
        ports
    
    }

    pub fn get_ip(&self)->Vec<IpAddr>{
        let mut ips = vec![];

        let splited = self.ip.split(',');

        for ip in splited{
            //resolve hostname
            ips.push(dns_lookup::lookup_host(ip).expect("ERROR : Couldn't resolve hostname")[0]);
        }

        //remove duplicates
        ips.sort();
        ips.dedup();
        ips
    }

    pub fn get_threads(self)->Option<u16>{
        self.threads
    }

}

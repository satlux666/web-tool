use colored::*;
use rayon::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use indicatif::{ProgressBar, ProgressStyle};
use crate::utils::domain_only;

pub fn port_scan(target: &str, port_str: &str, _threads: usize) -> Vec<u16> {
 let host = domain_only(target);
 println!("{} scanning ports on {}", "[*]".cyan(), host);

 let ports: Vec<u16> = if port_str == "top" {
 vec![21,22,23,25,53,80,110,111,135,139,143,443,445,993,995,1723,3306,3389,5900,8080,8443,8888,9000,9200,27017]
 } else if port_str == "full" {
 (1..1025).collect()
 } else {
 port_str.split(',').filter_map(|x| x.trim().parse().ok()).collect()
 };

 let open = Mutex::new(Vec::new());
 let pb = ProgressBar::new(ports.len() as u64);
 pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:35.cyan/blue}] {pos}/{len}").unwrap().progress_chars("=>-"));

 let t0 = Instant::now();
 ports.par_iter().for_each(|&p| {
 let addr = format!("{}:{}", host, p);
 if let Ok(mut iter) = addr.to_socket_addrs() {
 if let Some(sa) = iter.next() {
 if TcpStream::connect_timeout(&sa, Duration::from_millis(450)).is_ok() {
 open.lock().unwrap().push(p);
 }
 }
 }
 pb.inc(1);
 });
 pb.finish_and_clear();

 let mut res = open.into_inner().unwrap();
 res.sort();
 println!("{} {} open ports ({:.1}s)", "[+]".green(), res.len(), t0.elapsed().as_secs_f32());
 for p in &res {
 println!(" {}", p);
 }
 res
}

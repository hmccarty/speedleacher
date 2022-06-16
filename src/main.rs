extern crate exitcode;
extern crate regex;

extern crate serde_json;
extern crate serde_bencode;
//extern crate reqwest;

use serde::{Deserialize};

const MAX_KRPC_SIZE: usize = 128;

#[derive(serde::Deserialize)]
struct Config {
  udp_port: String,
  nodes: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct KRPCQuery {
  t: String, // Transaction ID
  y: char, // Message type
  q: String, // Method name
  a: serde_bencode::value::Value, // Method arguments
  v: String, // Client version
}

#[derive(serde::Deserialize)]
struct KRPCResponse {
  t: String, // Transaction ID
  y: char, // Message type
  r: serde_bencode::value::Value, // Method arguments
  v: String, // Client version
}

#[derive(serde::Deserialize)]
struct KRPCError {
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  // Parse args to collect magnet URI
  let mut magnet_uri: String = String::new(); 
  for arg in args {
    if arg.starts_with("magnet") {
      magnet_uri.push_str(&arg);
    }
  }
  if magnet_uri.is_empty() {
    eprintln!("Usage: ./main magnet_uri");
    std::process::exit(exitcode::DATAERR);
  }

  // Collect info hash and tracker from magnet URI
  let re: regex::Regex = regex::Regex::new(
    r"magnet:\?xt=urn:btih:(?P<ih>[[:alnum:]]{40})(\&dn=(?P<dn>.*))?").unwrap();
  if !re.is_match(&magnet_uri) {
    eprintln!("Invalid magnet URI");
    std::process::exit(exitcode::DATAERR);
  }

  let caps: regex::Captures = re.captures(&magnet_uri).unwrap();
  let info_hash: &str = caps.name("ih").map_or("", |m| m.as_str());
  let domain_name: &str = caps.name("dn").map_or("", |m| m.as_str());
  println!("Info hash: {}, Domain name: {}", info_hash, domain_name);

  // Generate peer ID
  let peer_id: &str = "harrisonswackyclient";

  // Collect peers from known nodes
  let config_file = std::fs::File::open("config.json")
    .expect("file should open read only");
  let config_reader = std::io::BufReader::new(config_file);
  let config: Config = serde_json::from_reader(config_reader)
    .expect("file should be proper JSON");
  println!("Peer: {}", &config.nodes[0]);

  let socket = std::net::UdpSocket::bind(format!("0.0.0.0:{}", config.udp_port))
    .expect("couldn't bind to address"); 
  socket.connect(&config.nodes[0]).expect("connect function failed");

  let mut ping_args = std::collections::HashMap::new();
  ping_args.insert("id".into(), peer_id.into());
  let ping_query = KRPCQuery {
    t: "aa".to_string(),
    y: 'q',
    q: "ping".to_string(),
    a: serde_bencode::value::Value::Dict(ping_args),
    v: "0.1.0".to_string(),
  };
  
  let msg = serde_bencode::to_string(&ping_query).unwrap();
  println!("Bencode ping: {}", msg);

  let data = serde_bencode::to_bytes(&ping_query).unwrap();
  socket.send(&data).expect("couldn't send message");

  let mut buf = [0; MAX_KRPC_SIZE];
  match socket.recv(&mut buf) {
    Ok(received) => println!("received {received} bytes {:?}", &buf[..received]),
    Err(e) => println!("recv function failed: {e:?}"),
  }

  // Send 'started' event to tracker via GET request
  // let client = reqwest::Client::new();
  //let resp = client.get(domain_name)
  //  .;

  // Parse response from tracker to get peers
  
  // Create TCP connection with each peer (store bits for choke and interest)
  
  // Send infohash to peer

  // 
}

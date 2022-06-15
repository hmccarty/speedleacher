extern crate exitcode;
extern crate regex;

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
  let re: regex::Regex = regex::Regex::new(r"magnet:\?xt=urn:btih:(?P<ih>[[:alnum:]]{40})(\&dn=(?P<dn>.*))?").unwrap();
  if !re.is_match(&magnet_uri) {
    eprintln!("Invalid magnet URI");
    std::process::exit(exitcode::DATAERR);
  }

  let caps: regex::Captures = re.captures(&magnet_uri).unwrap();
  let info_hash: &str = caps.name("ih").map_or("", |m| m.as_str());
  let domain_name: &str = caps.name("dn").map_or("", |m| m.as_str());

  println!("Info hash: {}, Domain name: {}", info_hash, domain_name);

  // Generate peer ID
  
  // Send 'started' event to tracker via GET request
  
  // Parse response from tracker to get peers
  
  // Create TCP connection with each peer (store bits for choke and interest)
  
  // Send infohash to peer

  // 
}

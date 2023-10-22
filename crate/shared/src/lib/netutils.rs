use regex::Regex;
use url::Url;

pub fn is_http_address(input: &str) -> bool {
  if let Ok(parsed_url) = Url::parse(input) {
    // Check if the scheme is 'http' or 'https'
    let scheme = parsed_url.scheme();
    return scheme == "http" || scheme == "https";
  }
  false
}

pub fn is_ip_with_port(input: &str) -> bool {
  let re =
    Regex::new(r"^(?P<ip>[0-9]{1,3}(\.[0-9]{1,3}){3}|::1|localhost):(?P<port>[0-9]+)$").unwrap();
  re.is_match(input)
}

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
    let re = Regex::new(r"^(?P<ip>[0-9]{1,3}(\.[0-9]{1,3}){3}|::1|localhost):(?P<port>[0-9]+)$")
        .unwrap();
    re.is_match(input)
}

// fn main() {
//     let test_string1 = "http://example.com";104.248.254.204:5000
//     let test_string2 = "https://example.com";
//     let test_string3 = "127.0.0.1:8080";
//     let test_string4 = "ftp://example.com";
//     let test_string5 = "localhost:3000";
//
//     println!("Is {} an HTTP address? {}", test_string1, is_http_address(test_string1));
//     println!("Is {} an HTTP address? {}", test_string2, is_http_address(test_string2));
//     println!("Is {} an IP with port? {}", test_string3, is_ip_with_port(test_string3));
//     println!("Is {} an HTTP address? {}", test_string4, is_http_address(test_string4));
//     println!("Is {} an IP with port? {}", test_string5, is_ip_with_port(test_string5));
// }104.248.254.204:5000

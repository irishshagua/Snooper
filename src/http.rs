use std::collections::HashMap;
use std::str::FromStr;
use httparse;

#[derive(Debug)]
pub enum HttpVerb {
  GET,
  POST,
  PUT,
  DELETE
}

impl FromStr for HttpVerb {
  type Err = ();

  fn from_str(s: &str) -> Result<HttpVerb, ()> {
    match &*s.to_lowercase() {
      "get" => Ok(HttpVerb::GET),
      "post" => Ok(HttpVerb::POST),
      "put" => Ok(HttpVerb::PUT),
      "delete" => Ok(HttpVerb::DELETE),
       _ => Err(()),
    }
  }
}

pub type HttpHeaders = HashMap<String, String>;
pub type RequestPath = String;
pub type RequestPayload = Option<String>;
pub type HttpRequest = (HttpVerb, RequestPath, RequestPayload, HttpHeaders);

pub struct HttpParser;

impl HttpParser {

  pub fn parse_http_request(buf: &[u8]) -> HttpRequest {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buf);

    (req.method.unwrap().parse::<HttpVerb>().unwrap(), req.path.unwrap().to_string(), None, HashMap::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn path_is_properly_extracted() {
    let input = b"GET http://www.rte.ie/sport HTTP/1.1\r\n\r\n";
    let http_req = HttpParser::parse_http_request(input);

    assert_eq!(http_req.1, "http://www.rte.ie/sport");
  }
}
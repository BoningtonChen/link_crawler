extern crate hyper;
extern crate url;

use std::io::Read;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::fmt;
use std::fmt::{format, Formatter};

use self::hyper::Client;
use self::hyper::status::StatusCode;
use self::url::{ParseResult, Url, UrlParser};

use hyper::header::parsing;

#[derive(Debug, Clone)]
pub enum UrlState {
	Accessible(Url),
	BadStatus(Url, StatusCode),
	ConnectionFailed(Url),
	TimedOut(Url),
	Malformed(String),
}

impl fmt::Display for UrlState {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			UrlState::Accessible(ref url) => format!("!! {}", url).fmt(f),
			UrlState::BadStatus(ref url, ref status) => format!("x {} ({})", url, status).fmt(f),
			UrlState::ConnectionFailed(ref url) => format!("x {} (Connection failed)", url).fmt(f),
			UrlState::TimedOut(ref url) => format!("x {} (Timed out)", url).fmt(f),
			UrlState::Malformed(ref url) => format!("x {} (Malformed)", url).fmt(f),
		}
	}
}

fn build_url(domain: &str, path: &str) -> ParseResult<Url> {
	let base_url_string = format!("https://{}", domain);    // 转换为https解析，原内容为http解析
	let base_url = Url::parse(&base_url_string).unwrap();
	
	let mut raw_url_parser = UrlParser::new();
	let url_parser = raw_url_parser.base_url(&base_url);
	
	url_parser.parse(path)
}

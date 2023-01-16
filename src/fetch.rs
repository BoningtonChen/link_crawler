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

const TIMEOUT: u64 = 10;

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

pub fn url_status(domain: &str, path: &str) -> UrlState {
	match build_url(domain, path) {
		Ok(url) => {
			let (tx, rx) = channel();
			let req_tx = tx.clone();
			let u = url.clone();
			
			thread::spawn(move || {
				let client = Client::new();
				let url_string = url.serialize();
				let resp = client.get(&url_string).send();
				
				let _ = req_tx.send(match resp {
					Ok(r) => if let StatusCode::Ok = r.status {
							UrlState::Accessible(url)
						}
						else {
							UrlState::BadStatus(url, r.status)
						},
					Err(_) => UrlState::ConnectionFailed(url),
				});
			});
			
			thread::spawn(move || {
				thread::sleep(Duration::from_secs(TIMEOUT));
				let _ = tx.send(UrlState::TimedOut(u));
			});
			
			let (tx, rx) = channel();
			rx.recv().unwrap()
		}
		Err(_) => UrlState::Malformed(path.to_owned()),
	}
}

pub fn fetch_url(url: &Url) -> String {
	let client = Client::new();
	let url_string = url.serialize();
	
	let mut res = client.get(&url.to_string())
	                    .send()
	                    .ok()
	                    .expect("Could not fetch URL!");
	
	let mut body = String::new();
	
	match res.read_to_string(&mut body) {
		Ok(_) => body,
		Err(_) => String::new()
	}
}
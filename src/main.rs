extern crate hyper;
extern crate openssl;

use std::env;
use std::io;
use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main() {
	println!("Hello random openssl world: {:?}", openssl::crypto::rand::rand_bytes(5));

	let url = match env::args().nth(1) {
		Some(url) => url,
		None => { println!("Usage: client <url>"); return; }
	};

	let client = Client::new();
	let res = client.get(&*url).header(Connection::close()).send().unwrap();

	println!("Response: {}", res.status);
	println!("Headers:\n{}", res.headers);
	io::copy(&mut res.take(50), &mut io::stdout()).unwrap();
	println!("[...]");
}

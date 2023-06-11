use reqwest;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Both subdomain and token must be specified.");
    }

    let url = format!(
        "https://www.duckdns.org/update?domains={}&token={}&verbose=true",
        args[1],
        args[2]
    );

    let response = reqwest::blocking::get(url)
        .expect("Something went wrong with the http request")
        .text()
        .expect("Error retrieving response body");

    println!("{:}", response);
}

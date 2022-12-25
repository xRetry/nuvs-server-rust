extern crate reqwest; // 0.9.18

use std::{thread, time::Duration};
use std::io::Read;

fn check_connection() -> Result<String, reqwest::Error> {
    let mut res = reqwest::get("http://127.0.0.1:2000/")?;
    
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let vec: Vec<&str> = body.split("\n").collect();

    return Ok(vec[0].to_string());
}

fn broadcast(message: String) {
    println!("Broadcasting: {}", message);
}

fn main() {
    loop {
        let conn_result = check_connection();
        let line = match conn_result {
            Ok(line) => line,
            Err(_) => continue
        };

        broadcast(line);
        thread::sleep(Duration::from_secs(10))
    }
}

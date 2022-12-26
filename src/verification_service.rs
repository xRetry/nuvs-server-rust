extern crate reqwest; // 0.9.18

use std::net::UdpSocket;
use std::{thread, time::Duration};
use std::io::Read;

fn check_connection() -> Result<String, reqwest::Error> {
    let mut res = reqwest::get("http://127.0.0.1:2000/")?;
    
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let vec: Vec<&str> = body.split("\n").collect();
    return Ok(vec[0].to_string());
}

fn broadcast(socket: &UdpSocket, message: String) {
    socket.send_to(message.as_bytes(), "10.0.0.255:2010")
        .expect("Unable to send data");
}

fn main() {
    let socket = UdpSocket::bind("10.0.0.255:2010")
        .expect("Unable to bind to broadcast address");
    socket.set_broadcast(true)
        .expect("Unable to set broadcast mode");
    
    loop {
        let conn_result = check_connection();
        let line = match conn_result {
            Ok(line) => line,
            Err(_) => continue
        };

        broadcast(&socket, line);
        thread::sleep(Duration::from_secs(10))
    }
}

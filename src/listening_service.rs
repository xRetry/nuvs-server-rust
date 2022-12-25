use std::net::UdpSocket;
use std::str;
use std::collections::HashMap;

#[derive(Debug)]
struct RecordEntry {
    ip: String,
    body: String,
}

impl RecordEntry {
    fn new(ip: String, body: String) -> RecordEntry {
        return RecordEntry{ ip, body };
    }
}

fn listen(mut map: HashMap<String, RecordEntry>) {
    let socket = UdpSocket::bind("10.0.0.255:2010")
        .expect("Unable to bind to address");

    let mut buf = [0; 1024];

    loop {
        let (n, src) = socket.recv_from(&mut buf)
            .expect("Error while receiving message");
        let buf = &mut buf[..n];
        let message = str::from_utf8(buf)
            .expect("Conversion error")
            .to_string();
        map.insert(src.to_string(), RecordEntry::new(src.to_string(), message));
        println!("{:?}", map);
    }
}

fn serve(map: &HashMap<String, RecordEntry>) {
    println!("{:?}", map);
}

fn main(){
    let mut map = HashMap::<String, RecordEntry>::new();
    serve(&map);
    listen(map);
}

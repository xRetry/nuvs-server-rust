extern crate serde_json;
extern crate serde;
extern crate tokio;

use std::str;
use std::collections::HashMap;
use std::{
    io,
    net::UdpSocket,
    sync::{RwLock, Arc},
};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct RecordEntry {
    ip: String,
    body: String,
    last_contact: i32,
}

impl RecordEntry {
    fn new(ip: String, body: String) -> RecordEntry {
        return RecordEntry{ ip, body, last_contact: 0};
    }
}

type RecordStorage = RwLock<HashMap<String, RecordEntry>>;


fn listen(records: &RecordStorage) {
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

        records.write().unwrap().insert(src.to_string(), RecordEntry::new(src.to_string(), message));
    }
}


#[tokio::main]
async fn start_http_service(records: &Arc<RecordStorage>) -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2020").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        let recs = Arc::clone(records);

        tokio::spawn(async move {
            serve_content(&mut stream, &recs).await;
            stream.shutdown().await.unwrap();
        });
    }
}


async fn serve_content(stream: &mut TcpStream, records: &RecordStorage) {
    let map = records.read().unwrap();

    let response = map.iter()
        .map(|(_, entry)| serde_json::to_string(entry).unwrap())
        .reduce(|a, b| a + "," + &b).unwrap();

    stream.try_write(response.as_bytes()).unwrap();
}


fn main(){
    let mut map = HashMap::<String, RecordEntry>::new();
    map.insert("sdsdf".to_string(), RecordEntry::new("sdsdf".to_string(), "esfefasefaasefas".to_string()));
    map.insert("asefasf".to_string(), RecordEntry::new("sdsafa".to_string(), "eiesgaegas".to_string()));
    let records = Arc::new(RwLock::new(map));
    start_http_service(&records).unwrap();
    //listen(&records);
}

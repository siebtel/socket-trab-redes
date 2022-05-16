use serde::{Deserialize, Serialize};
use std::io::stdin;
use std::net::UdpSocket;
use std::time::Instant;

#[derive(Debug)]
enum MessageType {
  Int,
  Char,
  String,
}

#[derive(Deserialize, Serialize)]
struct Message<MessageType> {
  r#type: String,
  val: MessageType,
}

fn define_type(raw_message: &str) -> MessageType {
  let message = raw_message.trim();

  if message.parse::<i32>().is_ok() {
    MessageType::Int
  } else if message.parse::<char>().is_ok() {
    MessageType::Char
  } else {
    MessageType::String
  }
}

fn send_to_socket(ip: &str, port: &str, message: &Vec<u8>) -> std::io::Result<()> {
  {
    let ip_and_port = format!("{}:{}", ip, port);
    println!("Connecting to: {}", ip_and_port);
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    let start = Instant::now();
    socket.send_to(message, ip_and_port)?;
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [32; 1024];

    let (amt, src) = socket.recv_from(&mut buf)?;
    // let end = Instant::now();
    // let round_trip_time = end.duration_since(start).expect("Time");

    let round_trip_time = start.elapsed();

    println!("round_trip_time {:?}", round_trip_time);
    println!("Received {} bytes from {}", amt, src);
    let response = std::str::from_utf8(&buf).unwrap();

    println!("Response {:?}", response.trim());
  } // the socket is closed here
  Ok(())
}

fn main() {
  loop {
    println!("Enter IP address: ");
    let mut ip_addr = String::new();
    stdin()
      .read_line(&mut ip_addr)
      .expect("Failed to read line");
    let ip_addr = ip_addr.trim();
    // --------------------------------------------------------------------------------
    println!("Enter number port: ");
    let mut port = String::new();
    stdin().read_line(&mut port).expect("Failed to read line");
    let port = port.trim();
    // --------------------------------------------------------------------------------

    println!("Enter message: ");
    let mut message = String::new();
    stdin()
      .read_line(&mut message)
      .expect("Failed to read line");
    // --------------------------------------------------------------------------------
    let message_type = define_type(&message);
    println!("Message type: {:?}", message_type);
    let message = Message {
      r#type: match message_type {
        MessageType::Int => "int".to_string(),
        MessageType::Char => "char".to_string(),
        MessageType::String => "string".to_string(),
      },
      val: message.trim().to_string(),
    };

    let message = serde_json::to_string(&message).unwrap().into_bytes();

    send_to_socket(ip_addr, port, &message).expect("Failed to send message");
  }
}
// {"type": "int", "val": "1"}

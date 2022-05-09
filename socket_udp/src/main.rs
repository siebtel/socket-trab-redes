use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)?;
    
            println!("Received {} bytes from {}", amt, src);
            println!("{:?}", buf);

            // what to do with the message
            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];

            buf.reverse();
            
            // send response
            socket.send_to(buf, &src)?;
        }
    } // the socket is closed here
    Ok(())
}
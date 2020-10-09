use std::error::Error;
use std::net::SocketAddr;
use std::{str, io};
use tokio::net::UdpSocket;
use byteorder::{ByteOrder, LittleEndian};
struct Server {
    socket: UdpSocket,
    buf: Vec<u8>
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            mut socket,
            mut buf
        } = self;

        loop {
            let result: Option<(usize, SocketAddr)>;
            result  = Some(socket.recv_from(&mut buf).await?);
            let byte_recv = result.unwrap().0;
            println!("Received: {}", byte_recv);
            // The first 5 bytes are 4 bytes DATA string with 1 byte blank
            let header = str::from_utf8(&buf[..4]).unwrap();
            println!("Header: {}", header);
            // Then the next 36 bytes are data, with the first 4 bytes is the column index
            let index = LittleEndian::read_u32(&buf[5..9]);
            println!("Index: {}", index);

            // The next 32 bytes are the values, make up to 8 float in totals. 4 bytes each
            let mut vec: Vec<f32> = Vec::new();

            for x in 0..8 {
                let start_index = (x*4) + 9;
                let end_index = start_index + 4;
                let temp = LittleEndian::read_f32(&buf[start_index..end_index]);
                vec.push(temp);
            }

            for val in vec{
                println!("Value: {}", val);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let socket = UdpSocket::bind("127.0.0.1:49005").await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 512]
    };

    // This starts the server task.
    server.run().await?;


    Ok(())
}
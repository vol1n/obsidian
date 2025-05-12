use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::io;
use std::collections::HashMap;
use std::io::Read;
const SERVER: Token = Token(0);

mod proto;
mod proto_schema;

struct ClientState {
    stream: TcpStream,
    buffer: Vec<u8>,
    read_buffer: [u8; 1024]
}

fn handle_packet(packet: &Vec<u8>) {
    println!("Got packet: ");
    println!("{:?}", packet);
    println!("Packet type:" );
}

fn main() -> io::Result<()> {
    let addr: SocketAddr = "127.0.0.1:25565".parse().unwrap();
    let mut listener = TcpListener::bind(addr)?;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut next_token_number: usize = 1;
    let mut clients: HashMap<Token, ClientState> = HashMap::new();
    poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;
    println!("Listening on {}", addr);
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    loop {
                        match listener.accept() {
                            Ok((mut stream, addr)) => {
                                println!("New connection from {}", addr);
                                let new_token = Token(next_token_number);
                                next_token_number += 1;
                                poll.registry().register(
                                    &mut stream,
                                    new_token,
                                    Interest::READABLE,
                                )?;
                                clients.insert(new_token, ClientState {
                                    stream: stream,
                                    read_buffer: [0u8; 1024],
                                    buffer: vec![]
                                });
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => return Err(e)
                        }
                    }
                },
                token => {
                    if event.is_readable() {
                        if let Some(client) = clients.get_mut(&token) {
                            match client.stream.read(&mut client.read_buffer) {
                                Ok(n) if n > 0 => {
                                    client.buffer.extend_from_slice(&client.read_buffer[..n]);
                                    if let Some((take_bytes, drop_bytes)) = proto::decode_varint(&client.buffer) {
                                        client.buffer = client.buffer[drop_bytes..].to_vec();
                                        println!("Received {} bytes from client {:?}", n, token);
                                        let packet: Vec<u8> = client.buffer.drain(..take_bytes as usize).collect();
                                        handle_packet(&packet);
                                        println!();
                                    } else {
                                        println!("Waiting for more bytes");
                                    }
                                    
                                }
                                Ok(0) => {
                                    // Probably connection closed
                                    println!("Client {:?} closed the connection", token);
                                    clients.remove(&token);
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // Spurious, nothing to do
                                }
                                Err(e) => {
                                    println!("Read error on {:?}: {}", token, e);
                                    clients.remove(&token);
                                },
                                Ok(_) => {}
                            }
                        }
                    }
                },
            }
        }
    }

}
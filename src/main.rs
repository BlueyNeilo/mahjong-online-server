/*
References:
https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers
https://en.wikipedia.org/wiki/Transmission_Control_Protocol
https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html
https://gist.github.com/andelf/8668088
https://stackoverflow.com/questions/17445485/example-tcp-server-written-in-rust
https://users.rust-lang.org/t/how-to-write-a-simple-tcp-client-and-server/3712/2
https://docs.rs/mio/0.6.11/mio/index.html

*/

//First meaningful github commit with code changed


//IP (Local host): 127.0.0.1
//TCP port used: 44405

//Metal IO
extern crate mio;
//Event Loop
use mio::*;

//TCP server
use std::net::{TcpListener, SocketAddr};
use mio::net::TcpStream;


fn main() {
    //Bind server socket
    let addr: SocketAddr = "127.0.0.1:0".parse()?;
    let server = TcpListener::bind(&addr)?;

    //Make events loop and a poll handle
    let poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);

    //Connect the stream
    let stream = TcpStream::connect(&server.local_addr()?)?;

    //Register stream with poll;
    poll.register(&stream, Token(0), Ready::readable() | Ready::writable(), PollOpt::edge())?;

    //Wait for socket to become ready.
    loop {
        poll.poll(&mut events, None)?;
        for event in &events {
            if event.token() == Token(0) && event.readiness().is_writable() {
                //Socket connected
                return Ok(());
            }
        }
    }
}


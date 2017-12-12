/*
References:
https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers
https://en.wikipedia.org/wiki/Transmission_Control_Protocol
https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html
https://gist.github.com/andelf/8668088
https://stackoverflow.com/questions/17445485/example-tcp-server-written-in-rust
https://users.rust-lang.org/t/how-to-write-a-simple-tcp-client-and-server/3712/2
https://docs.rs/mio/0.6.11/mio/index.html

Mahjong rules:
https://www.mastersofgames.com/rules/mah-jong-rules.htm
*/

/*
This server contains the Mahjong game state.
The game instance of each player communicates with this server.

Client requests:
-Wall state (Position of head, Position of tail)
-Current player (Relative to the client)
-Dead tiles state (Client gives number of dead tiles they have recieved. Server sends out the newest tiles up to the difference between dead tiles)
-Tile from head (When a player starts their turn)
-Tile from tail (For flowers and kong)

Server requests:
-Get live call (Requested of every player: chi, pong, kong)
-Get dead tile (When a player is to throw out a tile)
-Connection check
-Pause game
*/

//IP (Local host): 127.0.0.1
//Server TCP port used: 44405

mod mahjong_tiles;
use std::net::{TcpListener, TcpStream, SocketAddr};

struct PlayerConnection {
    connected: bool,
    spectating: bool,
    stream: TcpStream,
    addr: SocketAddr,
}

impl PlayerConnection {
    fn new(spectating : bool, stream: TcpStream, addr: SocketAddr) -> PlayerConnection {
        PlayerConnection {
            connected: true,
            spectating: spectating,
            stream: stream,
            addr: addr,
        }
    }

    //Not sure if this is how I want to set this up
    fn send() {
        //Communicating to the player
    }
    fn recieve() {
        //Communicating from the player
    }
}

fn main() {
    let server : TcpListener = TcpListener::bind("127.0.0.1:44405").unwrap();
    let mut connections : Vec<PlayerConnection> = Vec::with_capacity(4);

    for s in server.incoming() {
        match s {
            Ok((stream, addr)) => {
                let mut reconnecting = false;
                for connection in &mut connections {
                    if connection.addr==addr {
                        connection.connected = true;
                        connection.stream = stream;
                        reconnecting = true;
                        println!("Player reconnected! ({})", addr);
                        break;
                    }
                }
                if !reconnecting {
                    connections.push(PlayerConnection::new(
                        connections.len()>=4,
                        stream,
                        addr))
                }

                println!("New player! ({})", addr);
            }
            Err(e) => {
                println!("Connection failed.");
            }
        }
    }
}
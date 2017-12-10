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
*/

//IP (Local host): 127.0.0.1
//Server TCP port used: 44405
//Client TCP port used: 44406
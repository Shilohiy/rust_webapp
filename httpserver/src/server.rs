use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

/// Represents a server that listens for incoming connections and handles HTTP requests.
impl<'a> Server<'a> {
    /// Creates a new instance of the server with the specified socket address.
    ///
    /// # Arguments
    ///
    /// * `socket_addr` - The socket address to bind the server to.
    ///
    /// # Returns
    ///
    /// A new instance of the server.
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    /// Starts the server and listens for incoming connections.
    ///
    /// This method binds the server to the specified socket address and listens for incoming
    /// connections. For each incoming connection, it reads the request from the stream, converts
    /// it into an `HttpRequest`, and passes it to the router for further processing.
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Server running at {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut read_buffer = [0; 1024];
            //println!("read_buffer:{:?}", read_buffer);
            stream.read(&mut read_buffer).unwrap();

            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}

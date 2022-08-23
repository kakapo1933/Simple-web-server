use std::net::TcpListener;
use hello::request_handler;

fn main() {
    // When we declare listener with TcpListener::bind, we binding a listener to the socket address we specified.
    // And it listens for incoming TCP connections.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Iterate through all TCP connections and pass it to function created for handle them
    for stream in listener.incoming() {
        let stream = stream.unwrap();
 
        request_handler::handle_connection(stream);
    } 
}


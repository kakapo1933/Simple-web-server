use::std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // When we declare listener with TcpListener::bind, we binding a listener to the socket address we specified.
    // And it listens for incoming TCP connections.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Iterate through all TCP connections and pass it to function created for handle them
    for stream in listener.incoming() {
        let stream = stream.unwrap();
 
        handle_connection(stream);
    } 
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

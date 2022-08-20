use::std::{
    fs,
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
        // The browser signals the end of an HTTP request by sending two newline characters in a row,
        // so to get one request from the stream, we takes lines until we get a line that is empty string.
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

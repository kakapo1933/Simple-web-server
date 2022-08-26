use std::{
    net::TcpStream,
    thread,
    time::Duration,
    io::{prelude::*, BufReader},
    fs,
};

pub fn handle_connection(mut stream: TcpStream) {
    // below codes collect all information from request
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     // The browser signals the end of an HTTP request by sending two newline characters in a row,
    //     // so to get one request from the stream, we takes lines until we get a line that is empty string.
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let buf_reader = BufReader::new(&mut stream);
    let (request_method, request_path) = handle_request_information(buf_reader);

    let (status_line, file_name) = handle_route(&request_method, &request_path);

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_request_information(buf_reader: BufReader<&mut TcpStream>) -> (String, String) {
    let buf_reader: BufReader<&mut TcpStream> = buf_reader.into();
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_method = request_line.split(" ").collect::<Vec<&str>>()[0].to_string();
    let request_path = request_line.split(" ").collect::<Vec<&str>>()[1].to_string();
    (request_method, request_path)
}

fn handle_route <'a>(request_method: &'a str, request_path: &'a str) -> (&'a str, &'a str) {
    let (status_line, file_name) = match request_method {
        "GET" => {
            match request_path {
                "/" => ("HTTP/1.1 200 OK", "webPages/hello.html"),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK", "webPages/hello.html")
                },
                _ => ("HTTP/1.1 404 NOT FOUND", "webPages/404.html"),
            }
        },
        "POST" => ("HTTP/1.1 404 NOT FOUND", "webPages/404.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "webPages/404.html"),
    };
    (status_line, file_name)
}

#[cfg(test)]
mod tests {
    use crate::request_handler::handle_route;

    #[test]
    fn route_is_handled() {
        let (mut status_line, mut file_name) = handle_route("GET", "/");
        assert_eq!((status_line, file_name), ("HTTP/1.1 200 OK", "webPages/hello.html"));
        
        (status_line, file_name) = handle_route("POST", "/sleep");
        assert_eq!((status_line, file_name), ("HTTP/1.1 404 NOT FOUND", "webPages/404.html"));

        (status_line, file_name) = handle_route("ekokgo", "/oriu");
        assert_eq!((status_line, file_name), ("HTTP/1.1 404 NOT FOUND", "webPages/404.html"));
    }
}
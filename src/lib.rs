use std::net::TcpListener;

pub mod request_handler;

pub mod thread_handler;

/// Bind a socket to listen for
/// 
/// # Panic
/// 
/// create_litener will panic if socket binding failed
pub fn create_listener(socket_address: &str) ->  TcpListener {
    let listener = TcpListener::bind(socket_address).unwrap_or_else(|error| {
        panic!("{:?}", error);
    });
    listener
}
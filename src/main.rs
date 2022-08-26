use hello::{
    request_handler,
    create_listener,
    thread_handler,
};


fn main() {
    // When we declare listener with TcpListener::bind, we binding a listener to the socket address we specified.
    // And it listens for incoming TCP connections.
    let listener = create_listener("127.0.0.1:7878");
    // use thread pool to handle multiple request at the same time
    let pool = thread_handler::ThreadPool::new(4);
    // Iterate through all TCP connections and pass it to function created for handle them
    for stream in listener.incoming() {
        let stream = stream.unwrap();
 
        pool.execute(|| {
            request_handler::handle_connection(stream);
        });
    } 
}


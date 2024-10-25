use std::net::TcpListener;



mod tcp;

// Constants for server configuration
const HOST: &str = "127.0.0.1";
const PORT: &str = "8477";

fn main() {
    // Bind the server to the specified host and port
    let endpoint = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(endpoint).unwrap();

    println!("Web server is listening at port {}", PORT);

    // Iterate over incoming connections
    for incoming_stream in listener.incoming() {
        let mut stream = incoming_stream.unwrap();
        tcp::handle_connection(&mut stream); // Handle each connection
    }
}


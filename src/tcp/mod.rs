use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

const ROOT_DIR: &str = "www";

// Handles each client connection and processes the request
pub fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024]; // Buffer to read the request
    stream.read(&mut buffer).unwrap();

    // Convert the request buffer to a string
    let request_str = String::from_utf8_lossy(&buffer);

    // Parse the request path
    let request_path = parse_request_path(&request_str);

    // Serve the requested file based on the parsed path
    serve_requested_file(&request_path, stream);
}

// Extracts the requested path from the HTTP request
fn parse_request_path(request: &str) -> String {
    // Split the request and return the path part
    request.split_whitespace().nth(1).unwrap_or("/").to_string()
}

// Serves the file requested by the client
fn serve_requested_file(file_path: &str, stream: &mut TcpStream) {
    // Construct the full file path, defaults to index.html if "/" is requested
    let file_path = if file_path == "/" {
        format!("{}/index.html", ROOT_DIR)
    } else {
        format!("{}/{}", ROOT_DIR, &file_path[1..])
    };

    let path = Path::new(&file_path);

    // Generate the HTTP response based on file existence
    let response = match fs::read_to_string(&path) {
        // If the file exists, return its content with a 200 OK response
        Ok(contents) => format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        ),
        // If the file is not found, return a 404 Not Found response
        Err(_) => {
            let not_found = "404 Not Found.";
            format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                not_found.len(),
                not_found
            )
        }
    };

    // Send the response over the TCP stream to the client
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
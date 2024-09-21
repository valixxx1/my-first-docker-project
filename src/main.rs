use std::io::Write;

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let string = std::fs::read_to_string("index.html").unwrap();
        let content = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", string.len(), string);
        stream.write_all(content.as_bytes()).unwrap();
    }
}

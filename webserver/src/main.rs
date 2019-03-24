use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}

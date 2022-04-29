use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    println!("Send request!");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Invalid input!");
    stream.write(message.as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!("Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );


}

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let greet = "Hello, server!".as_bytes();

    if let Err(err) = socket.write_all(&greet).await {
        eprintln!("Write error: {err}");
    }
    let mut buffer = [0u8; 1024];
    let n = socket.read(&mut buffer).await.unwrap();
    let text = String::from_utf8_lossy(&buffer[..n]);
    println!("Text: {text}");
}

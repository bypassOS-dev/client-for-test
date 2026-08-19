use rustls::pki_types::ServerName;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio_rustls::{TlsConnector, client::TlsStream};
use std::io::BufReader;
use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let certs = load_certs();

    //Create an empty type of list (simple terms)
    let mut root_cert = rustls::RootCertStore::empty();

    //get all certificates from "cert.pem"
    for cert in certs {
        root_cert.add(cert)?;
    }

    //create TLS-client settings
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert) // We give OUR certificates (from "root_cert")
        .with_no_client_auth(); //Don't use client's certificates

    //Create TLS-connector using my config (TLS settings) 
    let connector = TlsConnector::from(Arc::new(config));

    //Get type "ServerName" | and give owned to "domain" ("try_from" crete a link)
    let domain = ServerName::try_from("localhost")?.to_owned();

    //just connect
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    //Runing Tls HandShake
    let mut tls_stream = connector.connect(domain, stream).await?;

    let greet = "Hello, server!".as_bytes();

    let mut buffer = [0u8; 1024];

    loop {
        tokio::select! {
            _ =  send_and_get(&mut tls_stream, greet, &mut buffer) => {
                println!("Your data was sent!");
            }
            _ = tokio::signal::ctrl_c() => {    //graceful shut down
                println!("Shutting down...");
                tls_stream.shutdown().await?;
                break;
            }
        }
    }
    Ok(())
}
fn load_certs() -> Vec<rustls::pki_types::CertificateDer<'static>>{
    //open file from path:
    let file = std::fs::File::open("cert.pem").unwrap();

    // "BufReader" better read file that just "File" so we use it:
    let mut reader = BufReader::new(file);

    // "rustls_pemfile" return iterator:
    rustls_pemfile::certs(&mut reader)
    .map(|cert| cert.unwrap())
    .collect()
}
async fn send_and_get(tls_stream: &mut TlsStream<TcpStream>, greet: &[u8], buffer: &mut [u8;1024]) {
    println!("Writing some...");
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    if let Err(err) = tls_stream.write_all(greet).await {
        eprintln!("Write error: {err}");
    }
        
    let n = tls_stream.read(buffer).await.unwrap();
    let text = String::from_utf8_lossy(&buffer[..n]);

    println!("Text: {text}");
}

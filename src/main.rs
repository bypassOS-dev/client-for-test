use rustls::pki_types::ServerName;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio_rustls::TlsConnector;
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

    if let Err(err) = tls_stream.write_all(greet).await {
        eprintln!("Write error: {err}");
    }

    let mut buffer = [0u8; 1024];
    let n = tls_stream.read(&mut buffer).await.unwrap();
    let text = String::from_utf8_lossy(&buffer[..n]);

    println!("Text: {text}");
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

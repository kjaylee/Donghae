use quinn::{Endpoint, ServerConfig};
use std::net::SocketAddr;
use log::info;
use futures::StreamExt;
use rustls::{Certificate, PrivateKey, ServerConfig as RustlsServerConfig};
use std::sync::Arc;

pub async fn start_quic_listener() {
    info!("Starting QUIC listener...");

    // Load TLS certificates and private key
    let cert = load_certs("cert.pem").expect("Failed to load certificate");
    let key = load_private_key("key.pem").expect("Failed to load private key");

    // Configure Rustls for TLS
    let rustls_config = RustlsServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth() // No client authentication
        .with_single_cert(cert, key)
        .expect("Failed to configure TLS");

    let server_config = ServerConfig::with_crypto(Arc::new(rustls_config));

    // Set listening address
    let addr: SocketAddr = "0.0.0.0:5000".parse().unwrap();

    // Create QUIC server endpoint
    let (endpoint, mut incoming) = Endpoint::server(server_config, addr).unwrap();

    // Spawn a task to handle incoming connections
    tokio::spawn(async move {
        while let Some(conn) = incoming.next().await {
            info!("Accepted new QUIC connection");
            tokio::spawn(handle_connection(conn));
        }
    });
}

async fn handle_connection(conn: quinn::Connecting) {
    // Handle the connection and extract the remote peer address
    let quinn::NewConnection { connection, .. } = conn.await.unwrap();
    info!("Connected Peer: {:?}", connection.remote_address());

    // Handle data streams or further processing here
    // For example, you could open a bi-directional stream and exchange messages.
    // ...
}

// Function to load the certificates from a file
fn load_certs(path: &str) -> Result<Vec<Certificate>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    use rustls_pemfile::certs;

    // Read the certificate file
    let cert_file = File::open(path)?;
    let mut reader = BufReader::new(cert_file);
    let certs = certs(&mut reader)?
        .into_iter()
        .map(Certificate)
        .collect();
    Ok(certs)
}

// Function to load the private key from a file
fn load_private_key(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    use rustls_pemfile::pkcs8_private_keys;

    // Read the private key file
    let key_file = File::open(path)?;
    let mut reader = BufReader::new(key_file);
    let mut keys = pkcs8_private_keys(&mut reader)?;

    // Return the first key found
    if !keys.is_empty() {
        Ok(PrivateKey(keys.remove(0)))
    } else {
        Err("No private key found.".into())
    }
}

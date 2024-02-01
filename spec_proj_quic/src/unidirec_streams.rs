#![cfg(feature = "rustls")]
use std::{error::Error, net::SocketAddr, sync::Arc};
use std::thread::sleep;
use std::time::Duration;
use anyhow;
use quinn::{ClientConfig, Connection, Endpoint};
use std::thread;
// use std::time::Duration;

// mod common;
use crate::common_util;

/// Runs a QUIC server bound to given address.
pub(crate) async fn run_server(addr: SocketAddr) -> anyhow::Result<()>{

    thread::sleep(Duration::from_secs(1));

    let (endpoint, _server_cert) = common_util::common_util::make_server_endpoint(addr).unwrap();
    // accept a single connection
    let incoming_conn = endpoint.accept().await.unwrap();
    let conn = incoming_conn.await.unwrap();

    // open_bidirectional_stream(conn.clone()).await;


    println!(
        "[server] connection accepted: addr={}",
        conn.remote_address()
    );

    let mut send = conn.open_uni().await?;

    send.write_all(b"test").await?;
    send.finish().await?;

    Ok(())

}

// async fn open_bidirectional_stream(connection: Connection) -> Result<(), dyn Error> {
//     let (mut send, mut recv) = connection
//         .open_bi()
//         .await?;
//
//     send.write_all(b"test").await?;
//     send.finish().await?;
//
//     let received = recv.read_to_end(10).await?;
//
//     Ok(())
// }


pub(crate) async fn run_client(server_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut endpoint = Endpoint::client("127.0.0.1:0".parse().unwrap())?;
    endpoint.set_default_client_config(configure_client());

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();

    let timeout_duration = Duration::from_secs(10);

    println!("[client] connected: addr={}", connection.remote_address());
    // Dropping handles allows the corresponding objects to automatically shut down

    while let Ok(mut recv) = connection.accept_uni().await {
        // Because it is a unidirectional stream, we can only receive not send back.
        println!("{:?}", recv.read_to_end(50).await?);
    }

    drop(connection);
    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;

    Ok(())
}

// async fn receive_bidirectional_stream(connection: Connection) -> Result<(), dyn Error> {
//     while let Ok((mut send, mut recv)) = connection.accept_bi().await {
//         // Because it is a bidirectional stream, we can both send and receive.
//         println!("request: {:?}", recv.read_to_end(50).await?);
//
//         send.write_all(b"response").await?;
//         send.finish().await?;
//     }
//
//     Ok(())
// }




/// Dummy certificate verifier that treats any certificate as valid.
/// NOTE, such verification is vulnerable to MITM attacks, but convenient for testing.
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}
#![cfg(feature = "rustls")]
use std::{error::Error};

// mod common;
mod common_util;
mod insec_conn;
mod unidirec_streams;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // server and client are running on the same thread asynchronously
//     let addr = "127.0.0.1:5000".parse().unwrap();
//     tokio::spawn(insec_conn::run_server(addr));
//     insec_conn::run_client(addr).await?;
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // server and client are running on the same thread asynchronously
    let addr = "127.0.0.1:5000".parse().unwrap();
    tokio::spawn(unidirec_streams::run_server(addr));
    unidirec_streams::run_client(addr).await?;
    Ok(())
}

#![cfg(feature = "rustls")]
use std::{error::Error};

// mod common;
mod common_util;
mod insec_conn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // server and client are running on the same thread asynchronously
    let addr = "127.0.0.1:5000".parse().unwrap();
    tokio::spawn(insec_conn::run_server(addr));
    insec_conn::run_client(addr).await?;
    Ok(())
}

#![cfg(feature = "rustls")]
use std::{error::Error};
use tokio::spawn;
// mod common;
mod common_util;
mod insec_conn;
mod unidirec_streams;
mod client;
mod server;

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


    // tokio::spawn(unidirec_streams::run_server(addr));
    // 创建并运行服务器任务
    let server_task = spawn(async move {
        unidirec_streams::run_server(addr).await.unwrap_or_else(|e| {
            eprintln!("Server error: {}", e);
        });
    });


    unidirec_streams::run_client(addr).await?;

    server_task.await?;

    Ok(())
}

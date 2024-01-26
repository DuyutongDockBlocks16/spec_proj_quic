use quinn::{Endpoint, ServerConfig, TransportConfig, CertificateChain, PrivateKey, RecvStream, SendStream};
use std::net::SocketAddr;
use std::fs::File;
use std::io::{self, Write};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 TLS 证书和私钥
    let certificate_chain = CertificateChain::from_pem(File::open("server.crt")?)?;
    let private_key = PrivateKey::from_pem(File::open("server.key")?)?;

    // 创建 QUIC 端点
    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout = Some(std::time::Duration::from_secs(5));
    let mut server_config = ServerConfig::default();
    server_config.transport = Arc::new(transport_config);
    server_config.crypto = Arc::new(server_config.crypto.clone().configure(|c| {
        c.certificate_chain = Some(certificate_chain);
        c.private_key = Some(private_key);
    }));
    let server_config = server_config;

    let addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let mut endpoint = Endpoint::builder();
    endpoint.listen(server_config, addr)?;

    println!("QUIC server listening on {}", addr);

    // 等待连接并处理请求
    let (mut endpoint, incoming) = endpoint.bind(&addr)?;
    while let Some(conn) = incoming.await.next().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(conn).await {
                eprintln!("Error: {:?}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(conn: quinn::NewConnection) -> Result<(), io::Error> {
    let quinn::NewConnection { connection, .. } = conn;
    let (mut send, mut recv) = connection.open_bi().await?;

    // 处理接收到的数据流
    while let Some(Ok(recv_stream)) = recv.next().await {
        tokio::spawn(async move {
            if let Err(e) = handle_stream(recv_stream).await {
                eprintln!("Error: {:?}", e);
            }
        });
    }

    Ok(())
}

async fn handle_stream(mut stream: RecvStream) -> Result<(), io::Error> {
    let mut data = Vec::new();
    while let Some(Ok(chunk)) = stream.read_to_end(1024).await {
        data.extend_from_slice(&chunk);
    }

    let request = String::from_utf8(data)?;
    println!("Received request: {}", request);

    let response = "Hello, QUIC client!".as_bytes();
    stream.write_all(response).await?;
    Ok(())
}

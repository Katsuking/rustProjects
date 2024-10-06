use anyhow::anyhow;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // port 8080に紐付ける
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            println!("new user connected!");
            _ = socket.write_all(b"Hello world").await;
        });
    }
    Ok(())
}

enum Input {
    Empty,
    Question(String),
}

async fn get_user_input(socket: &mut tokio::net::TcpStream) -> anyhow::Result<Input> {
    // 1024個のi32型要素を持つ固定配列 なので stackにある
    //
    let mut buffer = [0; 1024];
    let n = match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await? {
        Ok(0) => return anyhow::Error(anyhow!("Connection closed")),
        Ok(1) if buffer[0] == b'\n' => return anyhow::Ok(Input::Empty),
        Ok(2) if buffer[0] == b'\r' && buffer[1] == b'\n' => return anyhow::Ok(Input::Empty),
        Ok(n) => n,
        Err(e) => return anyhow::Err(anyhow!("Failed to read from socket: {}", e)),
    };
}

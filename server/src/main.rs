use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use common::{Action, Ds, Response, LAST};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7720);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, socket) = listener.accept().await?;
        println!("connected to {socket}");
        handle_connection(stream).await?;
    }
}

async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(stream);
    let mut recieved = Vec::new();
    buf_reader.read_until(LAST, &mut recieved).await?;
    let action = Action::from_bytes(recieved);
    println!("performing action :\n{:#?}", action);
    let mut stream = buf_reader.into_inner();
    let response = Response::Ls(vec![
        PathBuf::from("/home/eltahawy"),
        PathBuf::from("/root"),
    ])
    .bytes();
    stream.write_all(&response).await?;
    Ok(())
}

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use common::{Action, Ds};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7720);
    let mut stream = TcpStream::connect(addr).await?;
    let action = Action::Ls(PathBuf::from("/")).bytes();
    stream.write_all(&action).await?;
    Ok(())
}

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use common::{Action, Bytes, Reaction, Unit, LAST, PORT};

use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT);

pub async fn ls<T: Into<PathBuf>>(path: T) -> Result<Vec<Unit>, io::Error> {
    let action = Action::Ls(path.into());
    let result = action.dispatch().await?;
    match result {
        Reaction::Ls(vec) => Ok(vec),
        _ => unreachable!(),
    }
}

pub trait Dispatch {
    async fn dispatch(self) -> Result<Reaction, io::Error>;
}

impl Dispatch for Action {
    async fn dispatch(self) -> Result<Reaction, io::Error> {
        let mut stream = TcpStream::connect(ADDR).await?;
        let (r, mut w) = stream.split();
        w.write_all(&self.bytes()).await?;

        let mut b = BufReader::new(r);
        let mut recieved = Vec::new();
        b.read_until(LAST, &mut recieved).await?;
        let result = Reaction::from_bytes(recieved);
        Ok(result)
    }
}

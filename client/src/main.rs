use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use common::{Action, ActionResult, Bytes, Unit, LAST, PORT};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::sleep,
};

const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let units = ls("/home/eltahawy").await?;
        println!("{:#?}", units);
    }
}

async fn ls<T: Into<PathBuf>>(path: T) -> Result<Vec<Unit>, io::Error> {
    let action = Action::Ls(path.into());
    let result = make_request(action).await?;
    match result {
        ActionResult::Ls(vec) => Ok(vec),
        _ => unreachable!(),
    }
}

async fn make_request(action: Action) -> Result<ActionResult, io::Error> {
    let mut stream = TcpStream::connect(ADDR).await?;
    let (r, mut w) = stream.split();
    sleep(Duration::from_secs(3)).await;
    w.write_all(&action.bytes()).await?;

    let mut b = BufReader::new(r);
    let mut recieved = Vec::new();
    b.read_until(LAST, &mut recieved).await?;
    let result = ActionResult::from_bytes(recieved);
    println!("{:#?}", result);
    Ok(result)
}

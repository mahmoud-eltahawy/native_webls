use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use common::{Action, ActionResult, Unit, UnitKind, LAST, PORT};
use tokio::{
    fs::{self, remove_dir_all, remove_file},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT);
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
    let result = action.activate().await?;
    let mut stream = buf_reader.into_inner();
    stream.write_all(&result.bytes()).await?;
    Ok(())
}

trait Activate {
    async fn activate(&self) -> Result<ActionResult, io::Error>;
}

impl Activate for Action {
    async fn activate(&self) -> Result<ActionResult, io::Error> {
        let result = match self {
            Action::Ls(path_buf) => {
                let entries = ls(path_buf).await?;
                ActionResult::Ls(entries)
            }
            Action::Rm(vec) => {
                rm(vec).await?;
                ActionResult::Rm
            }
            Action::Mv { from, to } => ActionResult::Mv,
            Action::Cp { from, to } => ActionResult::Cp,
        };
        Ok(result)
    }
}

async fn rm(bases: &[Unit]) -> Result<(), io::Error> {
    let root = PathBuf::from_str("/home/eltahawy").unwrap();
    for base in bases.iter() {
        let path = root.join(base.path.clone());
        match base.kind {
            UnitKind::Dirctory => {
                remove_dir_all(path).await?;
            }
            _ => {
                remove_file(path).await?;
            }
        };
    }

    Ok(())
}

pub async fn ls(root: &PathBuf) -> Result<Vec<Unit>, io::Error> {
    let mut dir = fs::read_dir(&root).await?;
    let mut paths = Vec::new();
    while let Some(x) = dir.next_entry().await? {
        let kind = if x.file_type().await?.is_dir() {
            UnitKind::Dirctory
        } else {
            UnitKind::File
        };
        let unit = Unit {
            path: x.path().to_path_buf(),
            kind,
        };
        paths.push(unit);
    }

    Ok(paths)
}

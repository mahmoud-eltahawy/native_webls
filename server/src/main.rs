use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
    sync::LazyLock,
};

use common::{Action, Bytes, Reaction, Unit, UnitKind, LAST, PORT};
use tokio::{
    fs::{self, copy, remove_dir_all, remove_file},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    process::Command,
    task::JoinSet,
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
    let result = action.activate().await?;
    let mut stream = buf_reader.into_inner();
    stream.write_all(&result.bytes()).await?;
    Ok(())
}

trait Activate {
    async fn activate(self) -> Result<Reaction, io::Error>;
}

impl Activate for Action {
    async fn activate(self) -> Result<Reaction, io::Error> {
        let result = match self {
            Action::Ls(path_buf) => {
                let entries = ls(path_buf).await?;
                Reaction::Ls(entries)
            }
            Action::Rm(vec) => {
                rm(vec).await?;
                Reaction::Fine
            }
            Action::Mv { from, to } => {
                mv(from, to).await?;
                Reaction::Fine
            }
            Action::Cp { from, to } => {
                cp(from, to).await?;
                Reaction::Fine
            }
            Action::Mp4(vec) => {
                mp4_remux(vec).await?;
                Reaction::Fine
            }
        };
        Ok(result)
    }
}

static ROOT: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from_str("/home/eltahawy").unwrap());

async fn rm(units: Vec<Unit>) -> Result<(), io::Error> {
    for unit in units.into_iter() {
        let Unit { path, kind } = unit;
        let path = ROOT.join(path);
        match kind {
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

pub async fn ls(root: PathBuf) -> Result<Vec<Unit>, io::Error> {
    let root = ROOT.join(root);
    let mut dir = fs::read_dir(&root).await?;
    let mut paths = Vec::new();
    while let Some(x) = dir.next_entry().await? {
        let kind = if x.file_type().await?.is_dir() {
            UnitKind::Dirctory
        } else {
            UnitKind::File
        };
        let unit = Unit {
            path: x
                .path()
                .strip_prefix(ROOT.to_path_buf())
                .unwrap()
                .to_path_buf(),
            kind,
        };
        paths.push(unit);
    }
    Ok(paths)
}

async fn cp(from: Vec<PathBuf>, to: PathBuf) -> Result<(), io::Error> {
    let to = ROOT.join(to);
    let mut set = JoinSet::new();
    for path in from.iter().map(|x| ROOT.join(x)) {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        set.spawn(copy(path, to.join(name)));
    }

    while let Some(x) = set.join_next().await {
        let _ = x??;
    }

    Ok(())
}

async fn mv(from: Vec<PathBuf>, to: PathBuf) -> Result<(), io::Error> {
    let to = ROOT.join(to);
    let mut set = JoinSet::new();
    for base in from.into_iter().map(|x| ROOT.join(x)) {
        let name = base
            .file_name()
            .and_then(|x| x.to_str())
            .map(|x| x.to_string())
            .unwrap();
        set.spawn(cut(base, to.join(name)));
    }

    while let Some(x) = set.join_next().await {
        x??;
    }
    Ok(())
}

pub async fn cut(from: PathBuf, to: PathBuf) -> Result<(), io::Error> {
    copy(&from, to).await?;
    remove_file(from).await?;
    Ok(())
}

async fn mp4_remux(paths: Vec<PathBuf>) -> Result<(), io::Error> {
    let mut set = JoinSet::new();
    paths
        .into_iter()
        .map(|target| ROOT.join(target))
        .map(any_to_mp4)
        .for_each(|x| {
            set.spawn(x);
        });
    while let Some(x) = set.join_next().await {
        x??;
    }
    Ok(())
}

async fn any_to_mp4(from: PathBuf) -> Result<(), io::Error> {
    let mut to = from.clone();
    to.set_extension("mp4");
    let _ = remove_file(to.clone()).await;
    Command::new("ffmpeg")
        .arg("-i")
        .arg(from.clone())
        .arg(to)
        .spawn()?
        .wait()
        .await?;
    let _ = remove_file(from).await;
    Ok(())
}

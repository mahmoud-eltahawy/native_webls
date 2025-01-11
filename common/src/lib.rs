use ciborium::{de::from_reader, ser::into_writer};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;

pub const LAST: u8 = b'\n';
pub const PORT: u16 = 7720;

trait Message: Serialize + DeserializeOwned {}
pub trait Bytes {
    fn bytes(&self) -> Vec<u8>;
    fn from_bytes(data: Vec<u8>) -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Ls(PathBuf),
    Rm(Vec<Unit>),
    Mv { from: Vec<PathBuf>, to: PathBuf },
    Cp { from: Vec<PathBuf>, to: PathBuf },
    Mp4(Vec<PathBuf>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reaction {
    Ls(Vec<Unit>),
    Fine,
}
impl Message for Action {}
impl Message for Reaction {}

impl<T: Message> Bytes for T {
    fn bytes(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        into_writer(&self, &mut encoded).unwrap();
        encoded.push(LAST);
        encoded
    }

    fn from_bytes(mut data: Vec<u8>) -> Self {
        let _ = data.pop();
        let decoded: Self = from_reader(&data[..]).unwrap();
        decoded
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Unit {
    pub path: PathBuf,
    pub kind: UnitKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnitKind {
    Dirctory,
    Video,
    Audio,
    File,
}

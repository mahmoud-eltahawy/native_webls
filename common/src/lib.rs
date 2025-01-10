use ciborium::{de::from_reader, ser::into_writer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const LAST: u8 = b'\n';
pub const PORT: u16 = 7720;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Ls(PathBuf),
    Rm(Vec<Unit>),
    Mv { from: Vec<PathBuf>, to: PathBuf },
    Cp { from: Vec<PathBuf>, to: PathBuf },
}

impl Action {
    pub fn bytes(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        into_writer(&self, &mut encoded).unwrap();
        encoded.push(LAST);
        encoded
    }

    pub fn from_bytes(mut data: Vec<u8>) -> Self {
        let _ = data.pop();
        let decoded: Self = from_reader(&data[..]).unwrap();
        decoded
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionResult {
    Ls(Vec<Unit>),
    Rm,
    Mv,
    Cp,
}

impl ActionResult {
    pub fn bytes(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        into_writer(&self, &mut encoded).unwrap();
        encoded.push(LAST);
        encoded
    }

    pub fn from_bytes(mut data: Vec<u8>) -> Self {
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

use ciborium::{de::from_reader, ser::into_writer};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;

pub const LAST: u8 = b'\n';

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Ls(PathBuf),
    Rm(Vec<PathBuf>),
    Mv { from: Vec<PathBuf>, to: PathBuf },
    Cp { from: Vec<PathBuf>, to: PathBuf },
}

pub trait Ds {
    fn bytes(&self) -> Vec<u8>;
    fn from_bytes(data: Vec<u8>) -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ls(Vec<PathBuf>),
    Rm,
    Mv,
    Cp,
}

impl<T> Ds for T
where
    T: Serialize + DeserializeOwned,
{
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

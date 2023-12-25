use boss_monster_card_reader_core::CardInfos;
use serde::{ser::SerializeStruct, Serialize};
use std::{io::Write, path::Path};
use thiserror::Error;

struct SerdeCardInfos(CardInfos);

impl Serialize for SerdeCardInfos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("CardInfos", 3)?;
        s.serialize_field("name", &self.0.name)?;
        s.serialize_field("description", &self.0.description)?;
        s.end()
    }
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("The file type is unsupported")]
    UnsupportedType,
    #[error("IO error")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for WriteError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

pub fn write_disk(path: &Path, infos: Vec<CardInfos>) -> Result<(), WriteError> {
    if let Some(path) = path.parent() {
        std::fs::create_dir_all(path)?;
    }

    let infos: Vec<_> = infos.into_iter().map(|e| SerdeCardInfos(e)).collect();

    let buf = match path.extension().map(|e| e.to_str()).flatten() {
        Some("json") => {
            let buf = serde_json::to_string_pretty(&infos).unwrap();
            buf
        }
        _ => return Err(WriteError::UnsupportedType),
    };

    let mut file = std::fs::File::create(path)?;
    file.write_all(buf.as_bytes())?;

    Ok(())
}

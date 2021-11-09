use crate::DataSet;
use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

pub trait Load {
    type Error;
    fn load(self) -> Result<DataSet, Self::Error>;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Loader {
    // TODO: 支持其他格式
    Csv(CsvLoader),
}

impl Loader{
    pub fn load(self) -> Result<DataSet> {
        // TODO: 支持其他格式
        match self {
            Loader::Csv(csv) => csv.load(),
        }
    }
}

#[derive(Default, Debug)]
pub struct CsvLoader(pub(crate) String);

// 将CSV加载为DataSet
impl Load for CsvLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = CsvReader::new(Cursor::new(self.0))
            .infer_schema(Some(16))
            .finish()?;
        Ok(DataSet(df))
    }
}

// 内容检测
pub fn detect_content(data: String) -> Loader {
    // TODO: 内容检测
    Loader::Csv(CsvLoader(data))
}

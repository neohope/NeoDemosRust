use anyhow::{anyhow, Result};
use polars::prelude::*;
use sqlparser::parser::Parser;
use tracing::info;

mod nconvert;
mod ndialect;
mod nfetcher;
mod nloader;
mod ndataset;
use nconvert::Sql;
use ndataset::DataSet;
use nfetcher::retrieve_data;
use nloader::detect_content;

pub use ndialect::example_sql;
pub use ndialect::NDialect;

use std::convert::TryInto;

/// 从 from 中获取数据，从 where 中过滤，最后选取需要返回的列
pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    // 将sql解析为AST树结构
    let ast = Parser::parse_sql(&NDialect::default(), sql.as_ref())?;
    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    // 将AST转换为
    let sql = &ast[0];
    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}", source);

    // 从 source 读入一个CSV文件， 通过loader转成Dataset
    let ds = detect_content(retrieve_data(source).await?).load()?;

    // 根据condition，进行match运算
    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    // 根据order_by，做reduce运算，按col和desc排序
    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    // 通过offset和limit进行数据截取，也是选择操作的一种
    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }

    // 根据selection，做投影操作
    // 最后将LazyFrame转换为DataSet
    Ok(DataSet(filtered.select(selection).collect()?))
}

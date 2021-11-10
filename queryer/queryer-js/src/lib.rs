use neon::prelude::*;

pub fn example_sql(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(queryer::example_sql()))
}

fn query(mut cx: FunctionContext) -> JsResult<JsString> {
    //获取SQL
    let sql = cx.argument::<JsString>(0)?.value(&mut cx);
    //判断输出格式
    let output_fmt  = cx.argument_opt(1)
    .map_or(
        "csv".to_owned(),
        |v| v.to_string(&mut cx).unwrap().value(&mut cx),
    );

    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(async { queryer::query(sql).await.unwrap() });

    match output_fmt .as_str() {
        "csv" => Ok(cx.string(data.to_csv().unwrap())),
        v => cx.throw_type_error(format!("Output type {} not supported", v)),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("example_sql", example_sql)?;
    cx.export_function("query", query)?;
    Ok(())
}

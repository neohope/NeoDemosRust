use std::fs;

// 获取网页，转换为md文件处理异常
// scrape_url "https://www.rust-lang.org/" "rust.md"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //输出参数
	for arg in std::env::args() {
        println!("{}", arg);
    }

    // 参数校验
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        println!("Usage: scrape_url <url> <output file>");
        std::process::exit(1);
    }
    let url = &args[0];
    let output = &args[1];

    // 请求网页
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    // 转换为md
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    // 输出
    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    // 返回OK
    Ok(())
}

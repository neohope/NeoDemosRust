use std::fs;

//获取网页，转换为md文件，不处理异常
fn test(){
  let url = "https://www.rust-lang.org/";
  let output = "rust.md";
  
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}

// 获取网页，转换为md文件
// 函数现在返回一个 Result
fn test_exception()-> Result<(), Box<dyn std::error::Error>> {
  let url = "https://www.rust-lang.org/";
  let output = "rust.md";

  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url)?.text()?;

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes())?;
  println!("Converted markdown has been saved in {}.", output);

  Ok(())
}

fn main() {
  test();
  test_exception();
}
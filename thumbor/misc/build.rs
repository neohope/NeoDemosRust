use std::process::Command;

fn main() {
    //输出当前路径
    println!("{:?}", std::env::current_dir());

    /*
    // 避免重复生成protos
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }
    */

    // abi文件翻译为rs文件
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["misc/abi.proto"], &["."])
        .unwrap();

    // 格式化rs文件
    Command::new("cargo")
        .args(&["fmt", "--", "src/pb/abi.rs"])
        .status()
        .expect("cargo fmt failed");
}

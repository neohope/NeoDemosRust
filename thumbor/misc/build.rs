use std::process::Command;

fn main() {
    //输出当前路径
    println!("{:?}", std::env::current_dir());

    /*
    // 环境控制，避免重复生成
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }
    */

    prost_build::Config::new()
        .out_dir("../src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    
    Command::new("cargo")
        .args(&["fmt", "--", "rc/pb/*.rs"])
        .status()
        .expect("cargo fmt failed");
}

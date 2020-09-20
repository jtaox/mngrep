use std::env;
use std::process;

use mngrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("配置错误 {}", err);
        process::exit(1);
    });

    if let Err(e) = mngrep::run(config) {
        eprintln!("运行错误 {}", e);
        process::exit(1);
    }

}
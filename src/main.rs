//! ezsingbox - 简易sing-box 配置生成器和运行器

mod autoconfig;
mod commands;
mod config;
mod dns;
mod env;
mod protocol;
mod sharelink;
mod singboxconfig;
mod utils;

use std::process::ExitCode;

use commands::{cmd_generate, cmd_run, print_usage};

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _exe = args.next();
    let sub = args.next().unwrap_or_else(|| "generate".to_string());

    match sub.as_str() {
        "generate" => match cmd_generate() {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("❌ {}", e);
                ExitCode::from(1)
            }
        },
        "run" => match cmd_run() {
            Ok(code) => code,
            Err(e) => {
                eprintln!("❌ {}", e);
                ExitCode::from(1)
            }
        },
        // "serve" => match cmd_serve() {
        //     Ok(code) => code,
        //     Err(e) => {
        //         eprintln!("❌ {}", e);
        //         ExitCode::from(1)
        //     }
        // },
        _ => {
            print_usage();
            ExitCode::from(2)
        }
    }
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::PathBuf;
use lazy_static::lazy_static;
use crate::ss_node::{get_nodes, Node, write_config};

mod ss_local;
mod ss_node;

lazy_static! {
    /// 获取程序所在目录 并存储在静态变量中
    static ref PROGRAM_DIR: PathBuf = {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        exe_path.parent().expect("Failed to get program directory").to_path_buf()
    };
}



#[tauri::command]
fn get_nodes_from_url(link: &str) -> Result<Vec<Node>, String> {
    println!("============ url: {}", link);
    let nodes = get_nodes(link).map_err(|e| e.to_string())?;
    // 把这个订阅地址写入到文件中 以便下次启动时使用
    let subscription_url = PROGRAM_DIR.join("subscribe.txt");
    std::fs::write(subscription_url, link).map_err(|e| e.to_string())?;
    Ok(nodes)
}

#[tauri::command]
fn get_subscription_url_from_file() -> Result<String, String> {
    let subscription_url = PROGRAM_DIR.join("subscribe.txt");
    let url = std::fs::read_to_string(subscription_url).map_err(|_e| "".to_string())?;
    Ok(url)
}

#[tauri::command]
fn start_sslocal(node: Node) -> Result<String, String> {
    println!("============ node: {:?}", node);
    write_config(&node).map_err(|e| e.to_string())?;
    println!("config.json written");
    ss_local::start_sslocal().map_err(|e| e.to_string())?;
    Ok(format!("sslocal started at {}", node.local_port))
}

fn main() {
    println!("=====================");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_nodes_from_url,start_sslocal,get_subscription_url_from_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

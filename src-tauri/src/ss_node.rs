use crate::PROGRAM_DIR;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// 使用订阅连接(安卓版) 请求 ss 节点信息 并更新到界面 根据用户选择 启动 sslocal 并给出 socket链接 socks://127.0.0.1:1087 可以自定义 ip 端口
#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidNode {
    route: String,
    method: String,
    password: Option<String>,
    plugin: Option<String>,
    plugin_opts: Option<String>,
    remarks: Option<String>,
    server: Option<String>,
    server_port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub server: String,
    pub server_port: u16,
    pub password: String,
    pub method: String,
    pub plugin: String,
    #[serde(rename = "plugin-opts")]
    pub plugin_opts: String,
    pub local_address: String,
    pub local_port: u16,
}

impl AsRef<AndroidNode> for AndroidNode {
    fn as_ref(&self) -> &AndroidNode {
        self
    }
}

impl<T> From<T> for Node
    where
        T: AsRef<AndroidNode>,
{
    fn from(item: T) -> Self {
        let item = item.as_ref();
        Node {
            server: item.server.clone().unwrap_or_default(),
            server_port: item.server_port.unwrap_or_default(),
            password: item.password.clone().unwrap_or_default(),
            method: item.method.clone(),
            plugin: item.plugin.clone().unwrap_or_default(),
            plugin_opts: item.plugin_opts.clone().unwrap_or_default(),
            local_address: "127.0.0.1".into(),
            local_port: 1087,
        }
    }
}

/// 从安卓同款订阅链接中 获取节点信息 并转换成 sslocal 能够识别的格式
pub fn get_nodes(url: &str) -> anyhow::Result<Vec<Node>> {
    let resp_txt = reqwest::blocking::get(url)?.text()?;
    // 把x 转换成 vec[AndroidNode]
    let android_nodes = serde_json::from_str::<Vec<AndroidNode>>(&resp_txt)?;
    let nodes = android_nodes
        .iter()
        .map(|x| {
            let mut x = Node::from(x);
            let plugin_path = {
                #[cfg(windows)]
                {
                    let mut path = PROGRAM_DIR.join("v2ray-plugin");
                    path.set_extension("exe");
                    path
                }
                #[cfg(not(windows))]
                {
                    PROGRAM_DIR.join("v2ray-plugin")
                }
            };
            x.plugin = plugin_path.to_string_lossy().into_owned();
            x
        })
        .collect::<Vec<Node>>();
    Ok(nodes)
}

/// 写入配置文件到当前目录
pub fn write_config(node: &Node) -> anyhow::Result<()> {
    let node_str = serde_json::to_string(node)?;
    // 构建配置文件路径
    let config_path = PROGRAM_DIR.join("config.json");
    let mut file = File::create(config_path)?;
    file.write_all(node_str.as_bytes())?;
    Ok(())
}
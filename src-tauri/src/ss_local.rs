#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::io::Write;
use crate::PROGRAM_DIR;
use std::process::Command;

/// Windows下关闭正在运行的sslocal进程。
/// 使用taskkill命令强制关闭所有名为sslocal.exe的进程。
#[cfg(windows)]
fn kill_sslocal() -> anyhow::Result<()> {
    Command::new("taskkill")
        .args(&["/F", "/IM", "sslocal.exe"])
        .output()?;
    Ok(())
}

/// Unix系统下关闭正在运行的sslocal进程。
/// 使用pkill命令关闭所有名为sslocal的进程。
#[cfg(not(windows))]
fn kill_sslocal() -> anyhow::Result<()> {
    Command::new("pkill").arg("sslocal").output()?;
    Ok(())
}

#[cfg(not(windows))]
/// unix-like系统下启动sslocal进程。启动前会关闭已经在运行的sslocal进程，并使用当前目录下的config.json配置文件。
pub fn start_sslocal() -> anyhow::Result<()> {
    let _ = kill_sslocal(); // 忽略可能的错误
    let config_path = PROGRAM_DIR.join("config.json");
    let sslocal_path = PROGRAM_DIR.join("sslocal");
    Command::new(sslocal_path)
        .args(&[
            "-c",
            config_path
                .to_str()
                .ok_or_else(|| anyhow::Error::msg("Path to string conversion failed"))?,
        ])
        .spawn()?;
    Ok(())
}

#[cfg(windows)]
/// windows下启动sslocal进程。启动前会关闭已经在运行的sslocal进程，并使用当前目录下的config.json配置文件。
pub fn start_sslocal() -> anyhow::Result<()> {
    let _ = kill_sslocal(); // 忽略可能的错误
    let vbscript_path = PROGRAM_DIR.join("run_sslocal.vbs");
    let mut file = File::create(&vbscript_path)?;
    writeln!(file, "Set WshShell = CreateObject(\"WScript.Shell\")")?;
    writeln!(file, "WshShell.CurrentDirectory = \"{}\"", PROGRAM_DIR.to_str().ok_or_else(|| anyhow::Error::msg("String conversion failed"))?)?;
    writeln!(file, "WshShell.Run \"sslocal.exe -c config.json\", 0, False")?;
    Command::new("cscript")
        .arg(vbscript_path)
        .spawn()?;
    Ok(())
}

#[cfg(test)]
mod ss_local_tests {
    use super::*;

    #[test]
    fn test_kill_sslocal() {
        let _ = kill_sslocal().unwrap();
        println!("sslocal killed");
    }
}

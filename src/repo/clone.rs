use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 获取仓库的默认远程 URL
fn get_default_remote_url(dir: &Path) -> Option<String> {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

/// 生成克隆命令（保持原始协议类型）
pub fn gen_clone_commands(repos: Vec<PathBuf>) -> Vec<String> {
    let mut commands = Vec::new();
    
    for repo in repos {
        if let Some(url) = get_default_remote_url(&repo) {
            // 直接使用原始 URL，不进行转换
            commands.push(format!("git clone {}", url));
        } else {
            let repo_name = repo.file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_else(|| repo.to_string_lossy());
            commands.push(format!("# {} - no remote", repo_name));
        }
    }
    
    commands
}

/// 保存脚本到文件
pub fn save_script(path: &Path, cmds: &[String]) -> Result<(), String> {
    let mut content = String::from("#!/bin/bash\n\n");
    for cmd in cmds {
        content.push_str(cmd);
        content.push('\n');
    }
    
    fs::write(path, content)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    
    // 设置可执行权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|e| format!("获取文件权限失败: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .map_err(|e| format!("设置文件权限失败: {}", e))?;
    }
    
    Ok(())
}
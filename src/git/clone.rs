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

/// 将 SSH URL 转换为 HTTP URL
fn to_http(url: &str) -> String {
    if url.starts_with("git@") {
        // git@github.com:user/repo.git -> https://github.com/user/repo.git
        let without_prefix = url.strip_prefix("git@").unwrap();
        if let Some(colon_pos) = without_prefix.find(':') {
            let host = &without_prefix[..colon_pos];
            let path = &without_prefix[colon_pos + 1..];
            return format!("https://{}/{}", host, path);
        }
    } else if url.starts_with("ssh://git@") {
        // ssh://git@github.com/user/repo.git -> https://github.com/user/repo.git
        let without_prefix = url.strip_prefix("ssh://git@").unwrap();
        return format!("https://{}", without_prefix);
    }
    
    // 如果已经是 HTTP/HTTPS，直接返回
    url.to_string()
}

/// 将 HTTP URL 转换为 SSH URL
fn to_ssh(url: &str) -> String {
    if url.starts_with("https://") {
        // https://github.com/user/repo.git -> git@github.com:user/repo.git
        let without_prefix = url.strip_prefix("https://").unwrap();
        if let Some(slash_pos) = without_prefix.find('/') {
            let host = &without_prefix[..slash_pos];
            let path = &without_prefix[slash_pos + 1..];
            return format!("git@{}:{}", host, path);
        }
    } else if url.starts_with("http://") {
        // http://github.com/user/repo.git -> git@github.com:user/repo.git
        let without_prefix = url.strip_prefix("http://").unwrap();
        if let Some(slash_pos) = without_prefix.find('/') {
            let host = &without_prefix[..slash_pos];
            let path = &without_prefix[slash_pos + 1..];
            return format!("git@{}:{}", host, path);
        }
    }
    
    // 如果已经是 SSH，直接返回
    url.to_string()
}

/// 生成克隆命令
pub fn gen_clone_commands(repos: Vec<PathBuf>, transport: &str) -> Vec<String> {
    let mut commands = Vec::new();
    
    for repo in repos {
        if let Some(url) = get_default_remote_url(&repo) {
            let final_url = match transport {
                "ssh" => to_ssh(&url),
                _ => to_http(&url), // 默认使用 HTTP
            };
            commands.push(format!("git clone {}", final_url));
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
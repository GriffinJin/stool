use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

/// 获取配置目录路径
pub fn get_config_dir() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "synapse-cli") {
        let config_dir = proj_dirs.data_dir();
        
        // 确保配置目录存在
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .map_err(|e| format!("无法创建配置目录: {}", e))?;
        }
        
        Ok(config_dir.to_path_buf())
    } else {
        Err("无法获取配置目录路径".to_string())
    }
}

/// 获取工作区配置文件路径
fn get_workspaces_file() -> Result<PathBuf, String> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("workspaces.txt"))
}

/// 读取所有工作区
fn read_workspaces() -> Result<Vec<(String, PathBuf)>, String> {
    let workspaces_file = get_workspaces_file()?;
    
    if !workspaces_file.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&workspaces_file)
        .map_err(|e| format!("读取工作区配置失败: {}", e))?;
    
    let mut workspaces = Vec::new();
    for line in content.lines() {
        if let Some((name, path)) = line.split_once('=') {
            workspaces.push((name.to_string(), PathBuf::from(path)));
        }
    }
    
    Ok(workspaces)
}

/// 保存工作区配置
fn save_workspaces(workspaces: &[(String, PathBuf)]) -> Result<(), String> {
    let workspaces_file = get_workspaces_file()?;
    
    let content: String = workspaces
        .iter()
        .map(|(name, path)| format!("{}={}", name, path.display()))
        .collect::<Vec<_>>()
        .join("\n");
    
    fs::write(&workspaces_file, content)
        .map_err(|e| format!("保存工作区配置失败: {}", e))?;
    
    Ok(())
}

/// 创建新工作区
pub fn create_workspace() -> Result<(), String> {
    let cwd = env::current_dir()
        .map_err(|e| format!("无法获取当前目录: {}", e))?;
    
    let workspace_name = cwd
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("无法获取当前目录名称")?
        .to_string();
    
    let mut workspaces = read_workspaces()?;
    
    // 检查是否已存在
    if workspaces.iter().any(|(name, _)| name == &workspace_name) {
        return Err(format!("工作区 '{}' 已存在", workspace_name));
    }
    
    workspaces.push((workspace_name.clone(), cwd.clone()));
    save_workspaces(&workspaces)?;
    
    println!("✅ 成功创建工作区: {} -> {}", workspace_name, cwd.display());
    Ok(())
}

/// 初始化当前目录为工作区
pub fn init_workspace() -> Result<(), String> {
    create_workspace()
}

/// 列出所有工作区
pub fn list_workspaces() -> Result<(), String> {
    let workspaces = read_workspaces()?;
    
    if workspaces.is_empty() {
        println!("暂无工作区");
        return Ok(());
    }
    
    println!("工作区列表:");
    println!("{:<20} {}", "名称", "路径");
    println!("{}", "-".repeat(80));
    
    for (name, path) in workspaces {
        println!("{:<20} {}", name, path.display());
    }
    
    Ok(())
}

/// 切换到指定工作区
pub fn switch_workspace(workspace_name: &str) -> Result<(), String> {
    let workspaces = read_workspaces()?;
    
    let workspace = workspaces
        .iter()
        .find(|(name, _)| name == workspace_name)
        .ok_or(format!("工作区 '{}' 不存在", workspace_name))?;
    
    let path = &workspace.1;
    
    if !path.exists() {
        return Err(format!("工作区路径不存在: {}", path.display()));
    }
    
    // 在终端中，我们不能直接改变父进程的工作目录
    // 所以我们只能输出切换命令，让用户执行
    println!("请执行以下命令切换到工作区:");
    println!("cd {}", path.display());
    
    Ok(())
}

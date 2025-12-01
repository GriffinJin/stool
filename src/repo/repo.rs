use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 检查目录是否为 Git 仓库
pub fn is_git_repo(dir: &Path) -> bool {
    let git_path = dir.join(".git");
    git_path.exists()
}

/// 查找指定目录下的所有 Git 仓库（包括当前目录和一层子目录）
pub fn find_git_repos(base: &Path) -> Vec<PathBuf> {
    let mut repos = Vec::new();

    // 包含当前目录本身
    if is_git_repo(base) {
        repos.push(base.to_path_buf());
    }

    // 一层子目录
    if let Ok(entries) = fs::read_dir(base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && is_git_repo(&path) {
                repos.push(path);
            }
        }
    }

    repos
}

/// 执行 Git 命令并返回输出
fn run_git(dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// 仓库信息结构
#[derive(Debug)]
pub struct RepoInfo {
    pub name: String,
    pub path: String,
    pub branch: String,
    pub upstream: String,
    pub ahead: Option<u32>,
    pub behind: Option<u32>,
    pub changes: usize,
}

/// 获取单个仓库的详细信息
pub fn get_repo_info(dir: &Path) -> RepoInfo {
    let name = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| dir.to_string_lossy().to_string());
    let path = dir.to_string_lossy().to_string();

    // 分支名称（处理 detached HEAD）
    let mut branch = run_git(dir, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|| "-".to_string());
    if branch == "HEAD" {
        branch = run_git(dir, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "-".to_string());
    }

    // 上游分支（不扫描工作区）
    let upstream = run_git(dir, &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .unwrap_or_else(|| "-".to_string());

    // 领先/落后提交数
    let (ahead, behind) = if upstream != "-" {
        let ahead_behind = run_git(dir, &["rev-list", "--left-right", "--count", &format!("{}...@{{u}}", branch)])
            .unwrap_or_else(|| "0\t0".to_string());
        let parts: Vec<&str> = ahead_behind.split_whitespace().collect();
        let ahead = parts.get(0).and_then(|s| s.parse().ok());
        let behind = parts.get(1).and_then(|s| s.parse().ok());
        (ahead, behind)
    } else {
        (None, None)
    };

    // 工作区变更数量
    let changes = run_git(dir, &["status", "--porcelain"])
        .map(|output| output.lines().count())
        .unwrap_or(0);

    RepoInfo {
        name,
        path,
        branch,
        upstream,
        ahead,
        behind,
        changes,
    }
}

/// 并发获取多个仓库的信息
pub fn get_repos_info_parallel(repos: Vec<PathBuf>) -> Vec<RepoInfo> {
    // 临时实现：顺序收集信息
    // TODO: 后续可以优化为真正的并行收集
    let mut infos = Vec::with_capacity(repos.len());
    for repo in repos {
        infos.push(get_repo_info(&repo));
    }
    
    // 按名称排序
    infos.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    infos
}

/// 打印仓库信息表格
pub fn print_repos_table(infos: &[RepoInfo]) {
    if infos.is_empty() {
        println!("当前目录未发现 Git 仓库");
        return;
    }

    // 计算列宽
    let name_width = infos.iter().map(|info| info.name.len()).max().unwrap_or(4).max(4);
    let branch_width = infos.iter().map(|info| info.branch.len()).max().unwrap_or(6).max(6);
    let upstream_width = infos.iter().map(|info| info.upstream.len()).max().unwrap_or(8).max(8);

    // 打印表头
    println!(
        "{:<name_width$} {:<branch_width$} {:<upstream_width$} {:>6} {:>6} {:>7}",
        "NAME", "BRANCH", "UPSTREAM", "AHEAD", "BEHIND", "CHANGES",
        name_width = name_width,
        branch_width = branch_width,
        upstream_width = upstream_width
    );
    println!("{}", "-".repeat(name_width + branch_width + upstream_width + 26));

    // 打印数据行
    for info in infos {
        let ahead_str = info.ahead.map_or("-".to_string(), |n| n.to_string());
        let behind_str = info.behind.map_or("-".to_string(), |n| n.to_string());
        
        println!(
            "{:<name_width$} {:<branch_width$} {:<upstream_width$} {:>6} {:>6} {:>7}",
            info.name,
            info.branch,
            info.upstream,
            ahead_str,
            behind_str,
            info.changes,
            name_width = name_width,
            branch_width = branch_width,
            upstream_width = upstream_width
        );
    }
}
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::utils::parallel::ParallelExecutor;

/// Git pull 操作（静默模式）
fn git_pull_quiet(dir: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("pull")
        .arg("-q")
        .output()
        .map_err(|e| format!("执行 git 命令失败: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// Git fetch 操作（静默模式）
fn git_fetch_quiet(dir: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("fetch")
        .arg("-q")
        .output()
        .map_err(|e| format!("执行 git 命令失败: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// Git reset + clean 操作（静默模式）
fn git_reset_clean_quiet(dir: &Path) -> Result<(), String> {
    // git reset HEAD --hard -q
    let reset_output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("reset")
        .arg("HEAD")
        .arg("--hard")
        .arg("-q")
        .output()
        .map_err(|e| format!("执行 git reset 失败: {}", e))?;

    if !reset_output.status.success() {
        return Err(String::from_utf8_lossy(&reset_output.stderr).trim().to_string());
    }

    // git clean -fd -q
    let clean_output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("clean")
        .arg("-fd")
        .arg("-q")
        .output()
        .map_err(|e| format!("执行 git clean 失败: {}", e))?;

    if clean_output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&clean_output.stderr).trim().to_string())
    }
}

/// 检查本地分支是否存在
fn branch_exists_local(dir: &Path, branch: &str) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(&format!("refs/heads/{}", branch))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// 获取默认远程仓库名称
fn default_remote(_dir: &Path) -> String {
    "origin".to_string() // 简化实现，实际可以通过 git remote 获取
}

/// 检查远程分支是否存在
fn remote_branch_exists(dir: &Path, remote: &str, branch: &str) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(&format!("refs/remotes/{}/{}", remote, branch))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// 切换分支操作
fn switch_branch(dir: &Path, branch: &str, force: bool) -> Result<String, String> {
    if branch_exists_local(dir, branch) {
        // 本地分支存在，直接切换
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("checkout")
            .arg(branch)
            .output()
            .map_err(|e| format!("执行 git checkout 失败: {}", e))?;

        if output.status.success() {
            Ok(format!("切换到本地分支 {}", branch))
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    } else {
        // 本地分支不存在
        let remote = default_remote(dir);
        if remote_branch_exists(dir, &remote, branch) {
            // 远程分支存在，创建并切换
            let output = Command::new("git")
                .arg("-C")
                .arg(dir)
                .arg("checkout")
                .arg("-b")
                .arg(branch)
                .arg(&format!("{}/{}", remote, branch))
                .output()
                .map_err(|e| format!("执行 git checkout -b 失败: {}", e))?;

            if output.status.success() {
                Ok(format!("基于远程分支 {}/{} 创建并切换到 {}", remote, branch, branch))
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        } else if force {
            // 远程分支也不存在，但强制创建
            let output = Command::new("git")
                .arg("-C")
                .arg(dir)
                .arg("checkout")
                .arg("-b")
                .arg(branch)
                .output()
                .map_err(|e| format!("执行 git checkout -b 失败: {}", e))?;

            if output.status.success() {
                Ok(format!("强制创建并切换到新分支 {}", branch))
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        } else {
            Err(format!("分支 {} 不存在（本地和远程都没有），使用 --force 强制创建", branch))
        }
    }
}

/// 并发拉取所有仓库
pub fn pull_all_repos_parallel(repos: Vec<PathBuf>) {
    let errors = ParallelExecutor::execute(repos, |repo| {
        git_pull_quiet(repo).map_err(|e| {
            println!("[ERR] {:?}\n{}", repo, e);
            e
        })
    });

    if errors.is_empty() {
        println!("所有仓库拉取完成");
    }
}

/// 并发获取所有仓库的最新信息
pub fn fetch_all_repos_parallel(repos: Vec<PathBuf>) {
    let errors = ParallelExecutor::execute(repos, |repo| {
        git_fetch_quiet(repo)
    });

    for (repo, error) in errors {
        println!("[ERR] {:?}\n{}", repo, error);
    }
}

/// 并发切换所有仓库的分支
pub fn switch_all_repos_parallel(branch: &str, force: bool, repos: Vec<PathBuf>) {
    let branch = branch.to_string();
    let errors = ParallelExecutor::execute(repos, move |repo| {
        match switch_branch(repo, &branch, force) {
            Ok(msg) => {
                println!("[OK] {:?} - {}", repo, msg);
                Ok(())
            }
            Err(e) => {
                println!("[ERR] {:?} - {}", repo, e);
                Err(e)
            }
        }
    });

    if errors.is_empty() {
        println!("所有仓库分支切换完成");
    }
}

/// 并发清理所有仓库
pub fn clean_all_repos_parallel(repos: Vec<PathBuf>) {
    let errors = ParallelExecutor::execute(repos, |repo| {
        match git_reset_clean_quiet(repo) {
            Ok(()) => {
                println!("[OK] {:?} cleaned", repo);
                Ok(())
            }
            Err(e) => {
                println!("[ERR] {:?} - {}", repo, e);
                Err(e)
            }
        }
    });

    if errors.is_empty() {
        println!("所有仓库清理完成");
    }
}
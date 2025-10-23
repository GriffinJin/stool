use std::path::{Path, PathBuf};
use crate::utils::command::run_cmd_capture;

/// 使用 rg 查找包含指定字符串的文件
fn find_files_with_rg(base: &Path, needle: &str) -> Result<Vec<PathBuf>, String> {
    let output = run_cmd_capture("rg", &["-l", needle, &base.to_string_lossy()])?;
    let files = output
        .lines()
        .map(|line| PathBuf::from(line.trim()))
        .collect();
    Ok(files)
}

/// 使用 sd 在文件中进行替换
fn replace_with_sd(file: &Path, old: &str, new_: &str) -> Result<(), String> {
    let file_str = file.to_string_lossy();
    run_cmd_capture("sd", &[old, new_, &file_str])?;
    Ok(())
}

/// 版本替换主函数
pub fn version_replace(old: &str, new_: &str, base: &Path) -> Result<usize, String> {
    // 使用 rg 查找包含旧版本的文件
    let files = find_files_with_rg(base, old)?;
    
    if files.is_empty() {
        return Ok(0);
    }
    
    // 对每个文件进行替换
    let mut replaced_count = 0;
    for file in &files {
        match replace_with_sd(file, old, new_) {
            Ok(()) => {
                println!("替换文件: {}", file.display());
                replaced_count += 1;
            }
            Err(e) => {
                eprintln!("替换文件 {} 失败: {}", file.display(), e);
            }
        }
    }
    
    Ok(replaced_count)
}
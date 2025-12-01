use clap::Parser;
use std::env;
use std::path::Path;

use synapse_cli::{
    Cli, Commands, RepoCommands, VersionCommands, DbCommands,
    git::{find_git_repos, get_repos_info_parallel, print_repos_table, 
          pull_all_repos_parallel, fetch_all_repos_parallel, 
          switch_all_repos_parallel, clean_all_repos_parallel,
          gen_clone_commands, save_script},
    version::version_replace,
    db::rmid_file,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Repo { command } => match command {
            RepoCommands::Pull => {
                let cwd = env::current_dir().expect("无法获取当前目录");
                let repos = find_git_repos(&cwd);
                if repos.is_empty() {
                    println!("当前目录未发现 Git 仓库");
                    return;
                }
                pull_all_repos_parallel(repos);
            }
            RepoCommands::Ls { fetch } => {
                let cwd = env::current_dir().expect("无法获取当前目录");
                let repos = find_git_repos(&cwd);
                if repos.is_empty() {
                    println!("当前目录未发现 Git 仓库");
                    return;
                }

                if fetch {
                    println!("正在获取远程仓库信息...");
                    fetch_all_repos_parallel(repos.clone());
                }

                let infos = get_repos_info_parallel(repos);
                print_repos_table(&infos);
            }
            RepoCommands::Switch { branch, force } => {
                let cwd = env::current_dir().expect("无法获取当前目录");
                let repos = find_git_repos(&cwd);
                if repos.is_empty() {
                    println!("当前目录未发现 Git 仓库");
                    return;
                }
                switch_all_repos_parallel(&branch, force, repos);
            }
            RepoCommands::Genclone { save, path, transport } => {
                let cwd = env::current_dir().expect("无法获取当前目录");
                let repos = find_git_repos(&cwd);
                if repos.is_empty() {
                    println!("当前目录未发现 Git 仓库");
                    return;
                }

                let transport = transport.as_deref().unwrap_or("http");
                if transport != "http" && transport != "ssh" {
                    eprintln!("错误: transport 参数只支持 'http' 或 'ssh'");
                    return;
                }

                let commands = gen_clone_commands(repos, transport);

                if save {
                    let script_path = path.as_deref().unwrap_or("./clone_all.sh");
                    let script_path = Path::new(script_path);
                    
                    match save_script(script_path, &commands) {
                        Ok(()) => {
                            println!("克隆脚本已保存到: {}", script_path.display());
                        }
                        Err(e) => {
                            eprintln!("保存脚本失败: {}", e);
                        }
                    }
                } else {
                    for cmd in commands {
                        println!("{}", cmd);
                    }
                }
            }
            RepoCommands::Clean => {
                let cwd = env::current_dir().expect("无法获取当前目录");
                let repos = find_git_repos(&cwd);
                if repos.is_empty() {
                    println!("当前目录未发现 Git 仓库");
                    return;
                }
                clean_all_repos_parallel(repos);
            }
        },
        Commands::Version { command } => match command {
            VersionCommands::Replace { old_version, new_version } => {
                let cwd = env::current_dir().expect("无法获取当前目录");
                println!("正在替换版本号: {} -> {}", old_version, new_version);
                
                match version_replace(&old_version, &new_version, &cwd) {
                    Ok(count) => {
                        println!("版本替换完成，共替换 {} 个文件", count);
                    }
                    Err(e) => {
                        eprintln!("版本替换失败: {}", e);
                    }
                }
            }
        },
        Commands::Db { command } => match command {
            DbCommands::Rmid { sql_file_path } => {
                let path = Path::new(&sql_file_path);
                if !path.exists() {
                    eprintln!("文件不存在: {}", path.display());
                    std::process::exit(1);
                }
                match rmid_file(path) {
                    Ok(n) => {
                        println!("✅ 处理完成，共修改 {} 处（已生成备份：{}.bak）", n, path.file_name().unwrap().to_string_lossy());
                    }
                    Err(e) => {
                        eprintln!("处理失败: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}


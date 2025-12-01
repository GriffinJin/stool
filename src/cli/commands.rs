use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "st")]
#[command(version = "1.0")]
#[command(about = "Sunline Development Tools", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 仓库相关命令
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
    /// 版本号相关命令
    Version {
        #[command(subcommand)]
        command: VersionCommands,
    },
    /// 数据库相关命令
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum RepoCommands {
    Ls {
        /// 在列出信息前执行 git fetch 以获取最新远端信息
        #[arg(short, long)]
        fetch: bool,
        #[arg(short, long)]
        pull: bool,
        #[arg(short, long)]
        clean: bool,
    },
    /// 切换所有仓库到指定分支
    Switch {
        /// 分支名
        branch: String,
        /// 分支不存在时创建新分支
        #[arg(short, long)]
        force: bool,
    },
    /// 根据当前目录下一层的仓库生成批量 clone 命令
    Genclone {
        /// 保存为脚本文件（例如 -f ./clone_all.sh）
        #[arg(short = 'f', long = "file")]
        save: bool,
        /// 脚本保存路径（仅在 -f 时生效）
        path: Option<String>,
        /// 传输协议："http" 或 "ssh"，默认 http
        #[arg(short = 't', long = "transport")]
        transport: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum VersionCommands {
    /// 替换当前目录及子目录中包含旧版本号的文件内容
    Replace {
        /// 旧版本号
        old_version: String,
        /// 新版本号
        new_version: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// 清理 SQL 文件中 INSERT 的 id 字段及对应的首个值（原地修改，生成 .bak 备份）
    Rmid {
        /// SQL 文件路径
        sql_file_path: String,
    },
}
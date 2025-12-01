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
    /// 工作区相关命令
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommands,
    },
    /// 数据库相关命令
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum RepoCommands {
    /// 展示当前目录下所有的 Git 仓库
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
    /// 根据当前目录下一层的仓库生成批量 clone 命令（自动检测 HTTP/SSH）
    Genclone {
        /// 保存为脚本文件到当前目录的 clone.sh
        #[arg(short = 's', long = "save")]
        save: bool,
    },
    /// 替换当前目录及子目录中包含旧版本号的文件内容
    Updateversion {
        /// 旧版本号
        old_version: String,
        /// 新版本号
        new_version: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum WorkspaceCommands {
    /// 创建新工作区
    New,
    /// 初始化当前目录为工作区
    Init,
    /// 列出所有工作区
    Ls,
    /// 切换到指定工作区
    Cd {
        /// 工作区名称
        workspace_name: String,
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
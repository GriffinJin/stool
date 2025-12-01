pub mod repo;
pub mod operations;
pub mod clone;

pub use repo::{find_git_repos, is_git_repo, RepoInfo, get_repo_info, get_repos_info_parallel, print_repos_table};
pub use operations::{pull_all_repos_parallel, fetch_all_repos_parallel, switch_all_repos_parallel, clean_all_repos_parallel};
pub use clone::{gen_clone_commands, save_script};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

/// 并发执行器，用于并行处理多个任务
pub struct ParallelExecutor;

impl ParallelExecutor {
    /// 并发执行函数，对每个路径执行指定操作
    pub fn execute<F, E>(repos: Vec<PathBuf>, operation: F) -> Vec<(PathBuf, E)>
    where
        F: Fn(&PathBuf) -> Result<(), E> + Send + Sync + 'static,
        E: Send + 'static + std::fmt::Debug,
    {
        let errors: Arc<Mutex<Vec<(PathBuf, E)>>> = Arc::new(Mutex::new(Vec::new()));
        let repos_arc = Arc::new(repos);
        let idx = Arc::new(Mutex::new(0usize));
        let operation_arc = Arc::new(operation);

        let workers = std::cmp::min(
            repos_arc.len(),
            std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
        );
        let mut handles = Vec::with_capacity(workers);

        for _ in 0..workers {
            let repos_arc_cl = Arc::clone(&repos_arc);
            let idx_cl = Arc::clone(&idx);
            let errors_cl = Arc::clone(&errors);
            let operation_cl = Arc::clone(&operation_arc);
            
            let handle = thread::spawn(move || loop {
                let i = {
                    let mut guard = idx_cl.lock().unwrap();
                    let i = *guard;
                    *guard += 1;
                    i
                };
                if i >= repos_arc_cl.len() {
                    break;
                }
                let repo = &repos_arc_cl[i];
                if let Err(e) = operation_cl(repo) {
                    errors_cl.lock().unwrap().push((repo.clone(), e));
                }
            });
            handles.push(handle);
        }

        for h in handles {
            let _ = h.join();
        }

        Arc::try_unwrap(errors).unwrap().into_inner().unwrap()
    }
}
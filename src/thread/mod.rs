use anyhow::{Result, anyhow};
use std::thread;
use std::thread::JoinHandle;

pub trait JoinHandleExt {
    fn join_with_error(self, thread_name: &str) -> Result<()>;
}

impl JoinHandleExt for JoinHandle<Result<()>> {
    fn join_with_error(self, thread_name: &str) -> Result<()> {
        self.join()
            .map_err(|e| anyhow!("{} thread panicked: {:?}", thread_name, e))?
            .map_err(|e| anyhow!("{} failed: {}", thread_name, e))
    }
}

pub fn spawn_on_core<F, T>(core_id: usize, f: F) -> Result<JoinHandle<T>>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let core_ids = core_affinity::get_core_ids().ok_or(anyhow!("no cores found"))?;
    let core_id = *core_ids
        .get(core_id)
        .ok_or_else(|| anyhow!("Need at least {} CPU cores", core_id + 1))?;

    Ok(thread::spawn(move || {
        core_affinity::set_for_current(core_id);
        f()
    }))
}

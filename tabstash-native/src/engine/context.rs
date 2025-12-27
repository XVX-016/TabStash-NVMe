use std::path::PathBuf;
use std::sync::Arc;
use sled::Db;

pub struct EngineContext {
    pub snapshots_dir: PathBuf,
    pub db: Arc<Db>,
}

impl EngineContext {
    pub fn new(snapshots_dir: PathBuf, db: Arc<Db>) -> Self {
        Self { snapshots_dir, db }
    }
}


use sled::Db;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TabMetadata {
    pub tab_id: u32,
    pub url: String,
    pub timestamp: u64,
    pub size: u64,
    pub snapshot_id: String,
}

pub struct Index {
    db: Arc<Db>,
}

impl Index {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn insert(&self, metadata: &TabMetadata) -> Result<()> {
        let key = metadata.tab_id.to_le_bytes();
        let value = bincode::serialize(metadata)?;
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, tab_id: u32) -> Result<Option<TabMetadata>> {
        let key = tab_id.to_le_bytes();
        match self.db.get(key)? {
            Some(value) => {
                let metadata: TabMetadata = bincode::deserialize(&value)?;
                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    pub fn remove(&self, tab_id: u32) -> Result<()> {
        let key = tab_id.to_le_bytes();
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn list_all(&self) -> Result<Vec<TabMetadata>> {
        let mut results = Vec::new();
        for item in self.db.iter() {
            let (_, value) = item?;
            if let Ok(metadata) = bincode::deserialize::<TabMetadata>(&value) {
                results.push(metadata);
            }
        }
        Ok(results)
    }
}


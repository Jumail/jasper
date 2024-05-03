mod preferences_manager;

use anyhow::Error;
use async_trait::async_trait;
use teloxide::prelude::*;

use crate::{database::DatabaseManager, module_manager::Module};
pub(crate) use preferences_manager::PreferencesManager;

pub(crate) struct Prefs {
    db_mgr: DatabaseManager,
}

impl Prefs {
    pub(crate) fn new(db_mgr: DatabaseManager) -> Self {
        Self { db_mgr }
    }
}

#[async_trait]
impl Module for Prefs {
    async fn register_dependency(&mut self, dep_map: &mut DependencyMap) -> Result<(), Error> {
        let prefs_mgr = PreferencesManager::with_db_manager(self.db_mgr.clone()).await?;
        dep_map.insert(prefs_mgr);
        Ok(())
    }
}

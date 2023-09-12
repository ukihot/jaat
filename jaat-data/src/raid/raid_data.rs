use crate::Database;
use anyhow::Result;
use jaat_core::repository::raid_repository::IRaidRepository;

pub struct RaidRepositoryImpl {
    database: Database,
}

impl RaidRepositoryImpl {
    pub fn new(database: Database) -> RaidRepositoryImpl {
        RaidRepositoryImpl { database }
    }
}

impl IRaidRepository for RaidRepositoryImpl {
    fn find(&self, id: String) -> Result<Option<jaat_core::entity::raid::Raid>> {
        todo!()
    }

    fn save(&self, raid: jaat_core::entity::raid::Raid) -> Result<()> {
        todo!()
    }

    fn call_foo(&self) -> Result<String> {
        Ok("foo".to_string())
    }
}

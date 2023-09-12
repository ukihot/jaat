use anyhow::Result;

use crate::entity::raid::Raid;

pub trait IRaidRepository: Send + Sync + 'static {
    fn call_foo(&self) -> Result<String>;

    fn find(&self, id: String) -> Result<Option<Raid>>;

    fn save(&self, raid: Raid) -> Result<()>;
}

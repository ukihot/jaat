pub mod raid;
pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_hoge() -> &'static str {
        "hoge"
    }
}

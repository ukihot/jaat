use jaat_core::entity::player::Player;

pub struct PlayerDTO {
    pub name: String,
}

impl From<Player<'_>> for PlayerDTO {
    fn from(value: Player) -> Self {
        Self {
            name: value.name.into(),
        }
    }
}

use jaat_core::entity::player::Player;

pub struct PlayerDTO<'a> {
    pub gender: bool,
    pub height: Option<u8>,
    pub name: String,
    pub uniform_number: &'a str,
    pub weight: Option<u8>,
}

impl<'a> From<Player<'a>> for PlayerDTO<'a> {
    fn from(player: Player<'a>) -> Self {
        Self {
            name: player.name.into(),
            uniform_number: player.uniform_number,
            gender: player.gender.into(),
            height: player.height,
            weight: player.weight,
        }
    }
}

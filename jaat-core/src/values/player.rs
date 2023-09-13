use crate::entity::player::Player;
use garde::Validate;

#[derive(Debug)]
pub enum PlayerGender {
    Male,
    Female,
}

#[derive(Debug)]
pub enum PlayerStatus {
    Out,
    In,
}

#[derive(Debug, Validate)]
pub struct PlayerName<'a> {
    #[garde(ascii, length(min = 3, max = 25))]
    pub(crate) name: &'a str,
}

impl<'a> From<PlayerName<'a>> for String {
    fn from(player_name: PlayerName<'a>) -> Self {
        player_name.name.to_string()
    }
}

#[derive(Debug, Validate)]
pub struct PlayerId {
    #[garde(required)]
    pub(crate) id: Option<String>,
}

impl<'a> PartialEq for Player<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Playerの等価性をidによって比較します
        self.id == other.id
    }
}

impl PartialEq for PlayerId {
    fn eq(&self, other: &Self) -> bool {
        // PlayerIdの等価性をidによって比較します
        self.id == other.id
    }
}

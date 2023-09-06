use garde::Validate;

#[derive(Debug, Validate)]
pub struct Player<'a> {
    #[garde(skip)]
    pub gender: PlayerGender,
    #[garde(range(min = 140, max = 210))]
    pub height: Option<u8>,
    #[garde(skip)]
    pub id: PlayerId,
    #[garde(skip)]
    pub name: PlayerName<'a>,
    #[garde(skip)]
    pub status: PlayerStatus,
    #[garde(length(min = 1, max = 3))]
    pub uniform_number: Option<&'a str>,
    #[garde(range(min = 50, max = 85))]
    pub weight: Option<u8>,
}

impl<'a> Player<'a> {
    pub fn new(
        gender: PlayerGender,
        height: Option<u8>,
        id: PlayerId,
        name: PlayerName<'a>,
        status: PlayerStatus,
        uniform_number: Option<&'a str>,
        weight: Option<u8>,
    ) -> Self {
        Self {
            gender,
            height,
            id,
            name,
            status,
            uniform_number,
            weight,
        }
    }
}

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
    name: &'a str,
}

#[derive(Debug, Validate)]
pub struct PlayerId {
    #[garde(required)]
    id: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_equality() {
        let player1 = Player::new(
            PlayerGender::Female,
            Some(180),
            PlayerId {
                id: Some("1".to_string()),
            },
            PlayerName { name: "John" },
            PlayerStatus::In,
            Some("10"),
            Some(75),
        );

        let player2 = Player::new(
            PlayerGender::Female,
            Some(175),
            PlayerId {
                id: Some("1".to_string()),
            },
            PlayerName { name: "Alice" },
            PlayerStatus::In,
            Some("5"),
            Some(68),
        );

        assert_eq!(player1, player2);
    }

    #[test]
    fn test_player_notequality() {
        let player1 = Player::new(
            PlayerGender::Female,
            Some(180),
            PlayerId {
                id: Some("1".to_string()),
            },
            PlayerName { name: "John" },
            PlayerStatus::In,
            Some("4"),
            Some(75),
        );

        let player3 = Player::new(
            PlayerGender::Female,
            Some(175),
            PlayerId {
                id: Some("3".to_string()),
            },
            PlayerName { name: "Alice" },
            PlayerStatus::In,
            Some("9"),
            Some(68),
        );

        assert_ne!(player1, player3);
    }
}

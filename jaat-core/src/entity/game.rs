use crate::values::team::Team;
use garde::Validate;

#[derive(Debug, Validate)]
pub struct Game {
    #[garde(skip)]
    pub teams: [Team; 2],
    #[garde(skip)]
    pub first_team: Team,
}

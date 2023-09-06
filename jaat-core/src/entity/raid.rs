use std::time::Duration;
use super::player::Player;
use crate::values::raid_scenario::RaidScenario;
use garde::Validate;

#[derive(Debug, Validate)]
pub struct Raid<'a> {
    #[garde(skip)]
    pub anti: &'a [Player<'a>],
    #[garde(range(min = 1, max = 100))]
    pub cycle: u8,
    #[garde(skip)]
    pub duration: Duration,
    #[garde(range(min = 1, max = 3))]
    pub empty_count: u8,
    #[garde(skip)]
    pub is_bonus: bool,
    #[garde(skip)]
    pub raider: Player<'a>,
    #[garde(skip)]
    pub scenario: RaidScenario,
}

impl<'a> Raid<'a> {
    pub fn is_success(&self) -> bool {
        matches!(
            self.scenario,
            RaidScenario::OnlyBonus
                | RaidScenario::Empty
                | RaidScenario::HandTouch
                | RaidScenario::LowKick
                | RaidScenario::HighKick
                | RaidScenario::Parry
                | RaidScenario::Escape
                | RaidScenario::Chase
                | RaidScenario::AntiLineOut
        )
    }
}

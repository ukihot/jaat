use strum_macros::EnumString;

#[derive(Debug, EnumString, strum_macros::Display)]
pub enum RaidScenario {
    #[strum(serialize = "only_bonus")]
    OnlyBonus,
    #[strum(serialize = "empty")]
    Empty,
    #[strum(serialize = "hand_touch")]
    HandTouch,
    #[strum(serialize = "low_kick")]
    LowKick,
    #[strum(serialize = "high_kick")]
    HighKick,
    #[strum(serialize = "parry")]
    Parry,
    #[strum(serialize = "escape")]
    Escape,
    #[strum(serialize = "chase")]
    Chase,
    #[strum(serialize = "anti_line_out")]
    AntiLineOut,
    #[strum(serialize = "tackle")]
    Tackle,
    #[strum(serialize = "back_hold")]
    BackHold,
    #[strum(serialize = "ancle_catch")]
    AncleCatch,
    #[strum(serialize = "dive")]
    Dive,
    #[strum(serialize = "chain")]
    Chain,
    #[strum(serialize = "counter")]
    Counter,
    #[strum(serialize = "raider_line_out")]
    RaiderLineOut,
    #[strum(serialize = "dubki")]
    Dubki,
}

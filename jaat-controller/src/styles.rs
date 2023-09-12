mod grid;
mod radio;
mod reset_css;
mod text_form;
mod toggle;

pub struct Style {}

impl Style {
    pub const RESET: &str = reset_css::RESET;
    pub const TEXTFORM_STYLES: &str = text_form::TEXTFORM_STYLES;
    pub const GRID_STYLES: &str = grid::GRID_STYLES;
    pub const TOGGLE_STYLES: &str = toggle::TOGGLE_STYLES;
    pub const RADIO_STYLES: &str = radio::RADIO_STYLES;
}

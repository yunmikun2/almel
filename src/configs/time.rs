use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::color;
use crate::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_background")]
    pub background: Color,

    #[serde(default = "Config::default_foreground")]
    pub foreground: Color,

    #[serde(default = "Config::default_format")]
    pub format: String,

    #[serde(default = "Config::default_utc")]
    pub utc: bool,
}
impl Config {
    fn default_background() -> Color {
        color::GRAY
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_format() -> String {
        // nf-fa-clock_o
        "\u{f017} %Y/%m/%d %H:%M:%S".to_string()
    }
    fn default_utc() -> bool {
        false
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            format: Self::default_format(),
            utc: Self::default_utc(),
        }
    }
}
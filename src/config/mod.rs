use std::collections::HashMap;
use std::path::Path;
use std::fs::read as read_data;

use anyhow::Result;
use nightmare::events::Modifiers;
use serde::Deserialize;

use crate::input::Input;

mod actions;
mod parse;

use parse::parse_input;
pub use actions::Action;

// -----------------------------------------------------------------------------
//     - Config -
// -----------------------------------------------------------------------------
pub struct Config {
    actions: HashMap<(Input, Modifiers), Action>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let data = read_data(path)?;
        let cfg = toml::from_slice::<ConfigSrc>(&data)?;
        let inst = cfg.parse();
        Ok(inst)
    }

    pub fn key_map(&self, input: Input, modifiers: Modifiers) -> Action {
        *self.actions.get(&(input, modifiers)).unwrap_or(&Action::Noop)
    }
}

// -----------------------------------------------------------------------------
//     - Config source -
// -----------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct ConfigSrc {
    commands: Commands
}

impl ConfigSrc {
    fn parse(self) -> Config {
        let mut actions = HashMap::new();

        macro_rules! parse {
            ($field:ident, $action:ident) => {
                if let Ok(input) = parse_input(&self.commands.$field) {
                    actions.insert(input, Action::$action);
                }
            }
        }

        parse!(left, Left);
        parse!(right, Right);
        parse!(up, Up);
        parse!(down, Down);

        parse!(up_left, UpLeft);
        parse!(up_right, UpRight);
        parse!(down_left, DownLeft);
        parse!(down_right, DownRight);

        parse!(next_x_pixel, NextXPixel);
        parse!(prev_x_pixel, PrevXPixel);
        parse!(next_y_pixel, NextYPixel);
        parse!(prev_y_pixel, PrevYPixel);

        parse!(zoom_in, CanvasZoomIn);
        parse!(zoom_out, CanvasZoomOut);

        Config {
            actions,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Commands -
// -----------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Commands {
    left: String,
    right: String,
    up: String,
    down: String,

    up_left: String,
    up_right: String,
    down_left: String,
    down_right: String,

    canvas_left: String,
    canvas_right: String,
    canvas_up: String,
    canvas_down: String,

    zoom_in: String,
    zoom_out: String,

    next_x_pixel: String,
    next_y_pixel: String,
    prev_x_pixel: String,
    prev_y_pixel: String,

    visual: VisualCommands,
}

#[derive(Debug, Deserialize)]
pub struct VisualCommands {
    fill: String,
}

use std::collections::HashMap;
use std::path::Path;
use std::fs::read as read_data;

use anyhow::Result;
use nightmaregl::events::{Modifiers, Key};
use serde::Deserialize;

mod actions;

pub use actions::Action;

// -----------------------------------------------------------------------------
//     - Config -
// -----------------------------------------------------------------------------
pub struct Config {
    actions: HashMap<(Key, Modifiers), Action>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let data = read_data(path)?;
        let cfg = toml::from_slice::<ConfigSrc>(&data)?;
        let inst = cfg.make_magic();
        Ok(inst)
    }

    pub fn key_map(&self, key: Key, modifiers: Modifiers) -> Action {
        *self.actions.get(&(key, modifiers)).unwrap_or(&Action::Noop)
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
    fn make_magic(self) -> Config {
        let mut actions = HashMap::new();

        actions.insert((Key::L, Modifiers::empty()), Action::Left);

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

    visual: VisualCommands,
}

impl Commands {
    pub fn action(&self, key: Key, modifiers: Modifiers) -> Action {
        Action::Noop
    }
}

#[derive(Debug, Deserialize)]
pub struct VisualCommands {
    fill: String,
}

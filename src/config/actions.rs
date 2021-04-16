use serde::Deserialize;

// -----------------------------------------------------------------------------
//     - Actions -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,

    Noop,
}

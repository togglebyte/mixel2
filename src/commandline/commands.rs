#[derive(Debug)]
pub enum Command {
    Noop,
    Quit,
    Save { path: String, overwrite: bool },
    Extend(Extent),
    Put { x: i32, y: i32 }
}

#[derive(Debug, Default)]
pub struct Extent {
    pub left: i32,
    pub right: i32,
    pub up: i32,
    pub down: i32,
}

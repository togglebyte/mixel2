#[derive(Debug)]
pub enum Command {
    Noop,
    Quit,
    Save { path: String, overwrite: bool },
}

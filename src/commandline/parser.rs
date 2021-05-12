use log::info;
use nightmaregl::{Position, Size};

use super::commands::{Command, Extent};
use crate::canvas::Direction;

macro_rules! or_noop {
    ($e:expr) => {
        match $e {
            Some(val) => val,
            None => return Command::Noop
        }
    }
}

pub struct Parser<'a> {
    command: &'a str,
    args: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let command_end = src.find(' ').unwrap_or(src.len());
        let (command, args) = src.trim().split_at(command_end);
        Self {
            command: command.trim().get(1..).unwrap_or(""),
            args: args.trim(),
        }
    }

    pub fn parse(self) -> Command {
        macro_rules! extend {
            ($dir:ident) => {
                match self.args.parse::<i32>() {
                    Ok(n) => Command::Extend(Extent { $dir: n, ..Default::default() }),
                    Err(_) => Command::Noop,
                }
            }
        }

        info!("{:?} | {}", self.command, self.args);

        match self.command {
            "q" => Command::Quit,
            "w" => Command::Save { path: self.args.to_owned(), overwrite: false },
            "extendl" => extend!(left),
            "extendr" => extend!(right),
            "extendu" => extend!(up),
            "extendd" => extend!(down),
            "put" => Command::Put(or_noop!(self.args_to_pos())),
            "new" => Command::NewCanvas(or_noop!(self.args_to_size())),
            "split" => Command::Split(Direction::Horz),
            "splitv" => Command::Split(Direction::Vert),
            _ => Command::Noop,
        }
    }

    fn args_to_pos(&self) -> Option<Position<i32>> {
        let mut parts = self.args.split_whitespace();
        let x = parts.next().map(str::parse::<i32>).map(Result::ok).flatten()?;
        let y = parts.next().map(str::parse::<i32>).map(Result::ok).flatten()?;

        Some(Position::new(x, y))
    }

    fn args_to_size(&self) -> Option<Size<i32>> {
        let mut parts = self.args.split_whitespace();
        let width = parts.next().map(str::parse::<i32>).map(Result::ok).flatten()?;
        let height = parts.next().map(str::parse::<i32>).map(Result::ok).flatten()?;

        Some(Size::new(width, height))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quit() {
        let input = ":q";
        let output = matches!(Parser::new(input).parse(), Command::Quit);
        assert!(output);
    }

    #[test]
    fn save_without_path() {
        let input = ":w";
        let output = matches!(Parser::new(input).parse(), Command::Noop);
        assert!(output);
    }

//     #[test]
//     fn save() {
//         let input = ":w test.png";
//         let output = matches!(
//             Parser::parse(input),
//             Command::Save {
//                 overwrite: false,
//                 ..
//             }
//         );
//         assert!(output);
//     }

}

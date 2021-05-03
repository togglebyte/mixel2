use std::iter::Peekable;
use std::str::Chars;

use log::info;

use super::commands::{Command, Extent};

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

    pub fn parse(mut self) -> Command {
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
            "put" => {
                let mut florp = self.args.split_whitespace();
                let x = florp.next().map(str::parse::<i32>).map(Result::ok).flatten();
                let y = florp.next().map(str::parse::<i32>).map(Result::ok).flatten();

                match (x, y) {
                    (Some(x), Some(y)) => Command::Put { x, y },
                    _ => Command::Noop,
                }
                 

            }
            _ => Command::Noop,
        }
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

use log::info;
use nightmaregl::{Position, Size};
use nightmaregl::pixels::Pixel;

use crate::layout::Split;
use crate::canvas::LayerId;
use crate::plugins::{Arg, PluginCall};
use super::commands::Command;

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

        // TODO: this shouls resize
        //       all the textures in all the layers.
        //       Rename this to something other than extend
        // macro_rules! extend {
        //     ($dir:ident) => {
        //         match self.args.parse::<i32>() {
        //             Ok(n) => Command::Extend(Extent { $dir: n, ..Default::default() }),
        //             Err(_) => Command::Noop,
        //         }
        //     }
        // }

        info!("{:?} | {}", self.command, self.args);

        match self.command {
            "q" => Command::Quit,
            w@"w" | w@"w!" => Command::Save { path: self.args.to_owned(), overwrite: w == "w!" },
            // "extendl" => extend!(left),
            // "extendr" => extend!(right),
            // "extendu" => extend!(up),
            // "extendd" => extend!(down),
            "put" => Command::Put(or_noop!(self.args_to_pos())),
            "clear" => Command::Clear(or_noop!(self.args_to_pos())),
            "new" => Command::NewImage(or_noop!(self.args_to_size())),
            "split" => Command::Split(Split::Horz),
            "splitv" => Command::Split(Split::Vert),
            "close" => Command::CloseSelectedSplit,
            "colour" | "color" => Command::SetColour(or_noop!(self.args_to_rgb())),
            "alpha" => Command::SetAlpha(or_noop!(self.args_to_u8())),
            "layer" => Command::ChangeLayer(LayerId::from_display(or_noop!(self.args_to_usize()))),
            "newlayer" => Command::NewLayer,
            "removelayer" => Command::RemoveLayer,
            "lua" => Command::Lua(self.args.to_owned()),
            _ => Command::Noop,
        }
    }

    fn args_to_usize(&self) -> Option<usize> {
        self.args.parse::<usize>().ok()
    }

    fn args_to_u8(&self) -> Option<u8> {
        self.args.parse::<u8>().ok()
    }

    fn args_to_rgb(&self) -> Option<Pixel> {
        let mut parts = self.args.split_whitespace();
        let r = parts.next().map(str::parse::<u8>).map(Result::ok).flatten()?;
        let g = parts.next().map(str::parse::<u8>).map(Result::ok).flatten()?;
        let b = parts.next().map(str::parse::<u8>).map(Result::ok).flatten()?;

        let pixel = Pixel { r, g, b, a: 255, };

        Some(pixel)
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

    // #[test]
    // fn save_without_path() {
    //     let input = ":w";
    //     let output = matches!(Parser::new(input).parse(), Command::Noop);
    //     assert!(output);
    // }

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

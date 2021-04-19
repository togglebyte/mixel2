use std::iter::Peekable;
use std::str::Chars;

use super::commands::Command;

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let chars = src.chars().peekable();

        Self {
            chars,
        }
    }

    pub fn parseit(mut self) -> Command {
        Command::Noop

//         self.chars.next(); // Skip the `:` char.

//         loop {
//             let c = match self.chars.next() {
//                 Some(c) => c,
//                 None => return Command::Noop,
//             };

//             let next = self.chars.peek();
//             match (c, next) {
//                 // Single char commands
//                 ('q', _) => return Command::Quit,

//                 ('w', next)) => {
//                     let quit = *next == 'q';
//                     if quite {
//                         self.consume();
//                     }

//                     match *next {
//                         'q' => {
//                             self.consume();
//                         }
//                         '!' => {
//                             self.consume();
//                         }
//                         _ => {}
//                     }

//                     let path = self.string();
//                     if path.is_empty() {
//                         return Command::Noop;
//                     }
//                     return Command::Save{ overwrite, path };
//                 }
//                 // ('w', Some(nc @ ' ')) | ('w', Some(nc @ '!')) => {
//                 //     let overwrite = *nc == '!';
//                 //     let path = chars.skip_while(|c|c.is_whitespace()).collect::<String>();
//                 //     if path.is_empty() {
//                 //         return Command::Noop;
//                 //     }
//                 //     return Command::Save{ path, overwrite };
//                 // }
//                 _ => {}
//             }
//         }
    }

    fn string(self) -> String {
        self.chars.skip_while(|c|c.is_whitespace()).collect::<String>()
    }

    fn consume(&mut self) {
        drop(self.chars.next());
    }
}

impl<'a> Parser<'a> {
    pub fn parse(input: &str) -> Command {
        let mut chars = input.chars().peekable();
        chars.next();

        loop {
            let c = match chars.next() {
                Some(c) => c,
                None => return Command::Noop,
            };

            let next = chars.peek();
            match (c, next) {
                // Single char commands
                ('q', _) => return Command::Quit,

                // Double char commands
                ('w', Some('q')) => {
                    chars.next(); // consume 'q'
                    let overwrite = match chars.peek() {
                        Some('!') => true,
                        _ => false,
                    };
                    let path = chars.skip_while(|c|c.is_whitespace()).collect::<String>();
                    return Command::Save{ path, overwrite };
                }
                ('w', Some(nc @ ' ')) | ('w', Some(nc @ '!')) => {
                    let overwrite = *nc == '!';
                    let path = chars.skip_while(|c|c.is_whitespace()).collect::<String>();
                    if path.is_empty() {
                        return Command::Noop;
                    }
                    return Command::Save{ path, overwrite };
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quit() {
        let input = ":q";
        let output = matches!(Parser::parse(input), Command::Quit);
        assert!(output);
    }

    #[test]
    fn save_without_path() {
        let input = ":w";
        let output = matches!(Parser::parse(input), Command::Noop);
        assert!(output);
    }

    #[test]
    fn save() {
        let input = ":w test.png";
        let output = matches!(Parser::parse(input), Command::Save { overwrite: false, .. });
        assert!(output);
    }
}

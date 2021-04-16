use super::commands::Command;

pub struct Parser {
}

impl Parser {
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
                ('w', Some('q')) => {
                    chars.next(); // consume 'q'
                    let path = chars.skip_while(|c|c.is_whitespace()).collect::<String>();
                    return Command::Save{ path, overwrite: false };
                }
                ('w', Some(nc @ ' ')) | ('w', Some(nc @ '!')) => {
                    let overwrite = *nc == '!';
                    let path = chars.skip_while(|c|c.is_whitespace()).collect::<String>();
                    return Command::Save{ path, overwrite };
                }
                ('q', _) => return Command::Quit,
                _ => {}
            }
        }
    }
}

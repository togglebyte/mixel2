use super::commands::Command;

pub struct Parser {
}

impl Parser {
    pub fn parse(input: &str) -> Command {
        let mut chars = input.chars().peekable();
        chars.next();

        let mut command = Command::Noop;

        loop {
            let c = match chars.next() {
                Some(c) => c,
                None => return command,
            };

            let next = chars.peek();
            match (c, next) {
                ('w', Some('q')) => {
                    chars.next(); // consume 'q'
                    chars.next(); // consume space
                    let path = chars.collect::<String>();
                    return Command::Save{ path, overwrite: false };
                } // consume rest as the path, and skip the space 
                ('w', Some(' ')) => {
                    chars.next(); // consume space
                    let path = chars.collect::<String>();
                    eprintln!("path {:?}", path);
                    return Command::Save{ path, overwrite: false };
                } // consume rest as the path
                ('q', _) => return Command::Quit,
                _ => {}
            }
        }

        command
    }
}

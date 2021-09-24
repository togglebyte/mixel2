use nightmare::events::{Key, Modifiers};
use crate::input::Input;

fn keys_and_mods(keys: impl Iterator<Item=Input>) -> (Vec<Input>, Vec<Modifiers>) {
    let (keys, mods): (_, Vec<Input>) = keys.partition(|k| match k {
        Input::Key(Key::LControl) | Input::Key(Key::RControl) => false,
        Input::Key(Key::LAlt) | Input::Key(Key::RAlt) => false,
        Input::Key(Key::LShift) | Input::Key(Key::RShift) => false,
        _ => true,
    });

    let mods = mods.into_iter().filter_map(|k| match k {
        Input::Key(Key::LControl) | Input::Key(Key::RControl) => Some(Modifiers::CTRL),
        Input::Key(Key::LAlt) | Input::Key(Key::RAlt) => Some(Modifiers::ALT),
        Input::Key(Key::LShift) | Input::Key(Key::RShift) => Some(Modifiers::SHIFT),
        _ => None
    }).collect();

    (keys, mods)
}

// -----------------------------------------------------------------------------
//     - Parse input -
// -----------------------------------------------------------------------------
pub(super) fn parse_input(input: &str) -> Result<(Input, Modifiers), String> {
    if input.starts_with('<') && input.ends_with('>') {
        let input = &input[1..input.len() - 1];
        let keys = input.split('-').filter_map(str_to_key);
        let (keys, mods) = keys_and_mods(keys);
        
        // Modifiers
        let mut modifiers = Modifiers::empty();
        mods.into_iter().for_each(|m| modifiers.insert(m));

        match (input.contains("--"), keys.into_iter().next()) {
            (true, _) => Ok((Input::Char('-'), modifiers)),
            (false, Some(key)) => Ok((key, modifiers)),
            (false, None) => Err("No key mapping provided".to_owned()),
        }
    } else {
        match str_to_key(input) {
            Some(k) => Ok((k, Modifiers::empty())),
            None => Err(format!("\"{}\" is not a valid key", input))
        }
    }
}

fn str_to_key(s: &str) -> Option<Input> {
    match s {
        // Alpha
        "a" => Some(Input::Char('a')),
        "b" => Some(Input::Char('b')),
        "c" => Some(Input::Char('c')),
        "d" => Some(Input::Char('d')),
        "e" => Some(Input::Char('e')),
        "f" => Some(Input::Char('f')),
        "g" => Some(Input::Char('g')),
        "h" => Some(Input::Char('h')),
        "i" => Some(Input::Char('i')),
        "j" => Some(Input::Char('j')),
        "k" => Some(Input::Char('k')),
        "l" => Some(Input::Char('l')),
        "m" => Some(Input::Char('m')),
        "n" => Some(Input::Char('n')),
        "o" => Some(Input::Char('o')),
        "p" => Some(Input::Char('p')),
        "q" => Some(Input::Char('q')),
        "r" => Some(Input::Char('r')),
        "s" => Some(Input::Char('s')),
        "t" => Some(Input::Char('t')),
        "u" => Some(Input::Char('u')),
        "v" => Some(Input::Char('v')),
        "w" => Some(Input::Char('w')),
        "x" => Some(Input::Char('x')),
        "y" => Some(Input::Char('y')),
        "z" => Some(Input::Char('z')),

        // // Numbers
        // "0" => Some(Input::Char('0')),
        // "1" => Some(Input::Char('1')),
        // "1" => Some(Input::Char('1')),
        // "2" => Some(Input::Char('2')),
        // "3" => Some(Input::Char('3')),
        // "4" => Some(Input::Char('4')),
        // "5" => Some(Input::Char('5')),
        // "6" => Some(Input::Char('6')),
        // "7" => Some(Input::Char('7')),
        // "8" => Some(Input::Char('8')),
        // "9" => Some(Input::Char('9')),

        // Special chars
        "'" => Some(Input::Char('\'')),
        "*" => Some(Input::Char('*')),
        "@" => Some(Input::Char('@')),
        ":" => Some(Input::Char(':')),
        "=" => Some(Input::Char('=')),
        "-" => Some(Input::Char('-')),
        "+" => Some(Input::Char('+')),
        "." => Some(Input::Char('.')),
        "[" => Some(Input::Char('[')),
        "]" => Some(Input::Char(']')),
        ";" => Some(Input::Char(';')),
        "/" => Some(Input::Char('/')),
        "_" => Some(Input::Char('_')),
        "\\" => Some(Input::Char('\\')),

        "Tab" => Some(Input::Key(Key::Tab)),

        // Modifiers
        "C" => Some(Input::Key(Key::LControl)),
        "S" => Some(Input::Key(Key::LShift)),
        "A" => Some(Input::Key(Key::LAlt)),
        "Left" => Some(Input::Key(Key::Left)),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_key() {
        let actual = parse_input("a").unwrap();
        assert!(matches!(actual, (Input::Char('a'), _)));
    }

    #[test]
    fn test_parse_key_combination() {
        let (input, mods) = parse_input("<C-S-b>").unwrap();

        match input {
            Input::Char('b') => {
                assert!(mods.ctrl());
                assert!(mods.shift());
                assert!(!mods.alt());
            
            }
            _ => panic!("Invalid input"),
        }
    }

    #[test]
    fn test_directionals() {
        let actual = parse_input("<Left>").unwrap();
        assert!(matches!(actual, (Input::Key(Key::Left), _)));
    }

    #[test]
    fn test_directionals_with_modifier() {
        let input = parse_input("<S-Left>").unwrap();
        match input {
            (Input::Key(Key::Left), mods) => {
                assert!(mods.shift());
                assert!(!mods.ctrl());
                assert!(!mods.alt());
            
            }
            _ => panic!("Invalid input"),
        }
    }
}

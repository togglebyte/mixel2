use nightmaregl::events::{Key, Modifiers};
use crate::input::Input;

fn keys_and_mods(keys: impl Iterator<Item=Key>) -> (Vec<Key>, Vec<Modifiers>) {
    let (keys, mods): (_, Vec<Key>) = keys.partition(|k| match k {
        Key::LControl | Key::RControl => false,
        Key::LAlt | Key::RAlt => false,
        Key::LShift | Key::RShift => false,
        _ => true,
    });

    let mods = mods.into_iter().filter_map(|k| match k {
        Key::LControl | Key::RControl => Some(Modifiers::CTRL),
        Key::LAlt | Key::RAlt => Some(Modifiers::ALT),
        Key::LShift | Key::RShift => Some(Modifiers::SHIFT),
        _ => None
    }).collect();

    (keys, mods)
}

// -----------------------------------------------------------------------------
//     - Parse input -
// -----------------------------------------------------------------------------
pub(super) fn parse_input(input: &str) -> Result<Input, String> {
    if input.starts_with('<') && input.ends_with('>') {
        let input = &input[1..input.len() - 1];
        let keys = input.split('-').filter_map(str_to_key);
        let (keys, mods) = keys_and_mods(keys);
        
        // Modifiers
        let mut modifiers = Modifiers::empty();
        mods.into_iter().for_each(|m| modifiers.insert(m));

        match (input.contains("--"), keys.into_iter().next()) {
            (true, _) => Ok(Input::Key(Key::Minus, modifiers)),
            (false, Some(key)) => Ok(Input::Key(key, modifiers)),
            (false, None) => Err("No key mapping provided".to_owned()),
        }
    } else {
        match str_to_key(input) {
            Some(k) => Ok(Input::Key(k, Modifiers::empty())),
            None => Err(format!("\"{}\" is not a valid key", input))
        }
    }
}

fn str_to_key(s: &str) -> Option<Key> {
    match s {
        // Alpha
        "a" => Some(Key::A),
        "b" => Some(Key::B),
        "c" => Some(Key::C),
        "d" => Some(Key::D),
        "e" => Some(Key::E),
        "f" => Some(Key::F),
        "g" => Some(Key::G),
        "h" => Some(Key::H),
        "i" => Some(Key::I),
        "j" => Some(Key::J),
        "k" => Some(Key::K),
        "l" => Some(Key::L),
        "m" => Some(Key::M),
        "n" => Some(Key::N),
        "o" => Some(Key::O),
        "p" => Some(Key::P),
        "q" => Some(Key::Q),
        "r" => Some(Key::R),
        "s" => Some(Key::S),
        "t" => Some(Key::T),
        "u" => Some(Key::U),
        "v" => Some(Key::V),
        "w" => Some(Key::W),
        "x" => Some(Key::X),
        "y" => Some(Key::Y),
        "z" => Some(Key::Z),

        // Numbers
        "0" => Some(Key::Key0),
        "1" => Some(Key::Key1),
        "1" => Some(Key::Key1),
        "2" => Some(Key::Key2),
        "3" => Some(Key::Key3),
        "4" => Some(Key::Key4),
        "5" => Some(Key::Key5),
        "6" => Some(Key::Key6),
        "7" => Some(Key::Key7),
        "8" => Some(Key::Key8),
        "9" => Some(Key::Key9),

        // Special chars
        "'" => Some(Key::Apostrophe),
        "*" => Some(Key::Asterisk),
        "@" => Some(Key::At),
        "\\" => Some(Key::Backslash),
        ":" => Some(Key::Colon),
        "=" => Some(Key::Equals),
        "-" => Some(Key::Minus),
        "+" => Some(Key::Plus),
        "." => Some(Key::Period),
        "]" => Some(Key::RBracket),
        "[" => Some(Key::LBracket),
        ";" => Some(Key::Semicolon),
        "/" => Some(Key::Slash),
        "_" => Some(Key::Underline),
        "Tab" => Some(Key::Tab),

        // Modifiers
        "C" => Some(Key::LControl),
        "S" => Some(Key::LShift),
        "A" => Some(Key::LAlt),
        "Left" => Some(Key::Left),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_key() {
        let actual = parse_input("a").unwrap();
        assert!(matches!(actual, Input::Key(Key::A, _)));
    }

    #[test]
    fn test_parse_key_combination() {
        let input = parse_input("<C-S-b>").unwrap();

        match input {
            Input::Key(Key::B, mods) => {
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
        assert!(matches!(actual, Input::Key(Key::Left, _)));
    }

    #[test]
    fn test_directionals_with_modifier() {
        let input = parse_input("<S-Left>").unwrap();
        match input {
            Input::Key(Key::Left, mods) => {
                assert!(mods.shift());
                assert!(!mods.ctrl());
                assert!(!mods.alt());
            
            }
            _ => panic!("Invalid input"),
        }
    }
}

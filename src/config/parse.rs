use nightmaregl::events::{Key, Modifiers};

// -----------------------------------------------------------------------------
//     - Parse input -
// -----------------------------------------------------------------------------
pub(super) fn parse_input(input: &str) -> Result<(Key, Modifiers), String> {
    let chars = input.chars().peekable();

    let mut modifiers = Modifiers::empty();

    let keys = chars
        .take_while(|c| *c != '>')
        .filter(|c| *c != '-')
        .collect::<Vec<char>>();

    // Modifiers
    keys.iter().for_each(|c| {
        match c {
            'S' => modifiers.insert(Modifiers::SHIFT),
            'C' => modifiers.insert(Modifiers::CTRL),
            'A' => modifiers.insert(Modifiers::ALT),
            'A'..='Z' => {} // Ignore all other possible modifiers
            c => {}
        }
    });

    // Key
    let key = keys
        .iter()
        .filter_map(|c| match *c {
            'a'..='z' => char_to_key_code(*c),
            _ => None,
        })
        .next();

    match key {
        Some(k) => Ok((k, modifiers)),
        None => Err("Invalid keys".to_string()),
    }
}

fn char_to_key_code(c: char) -> Option<Key> {
    match c {
        'a' => Some(Key::A),
        'b' => Some(Key::B),
        'c' => Some(Key::C),
        'd' => Some(Key::D),
        'e' => Some(Key::E),
        'f' => Some(Key::F),
        'g' => Some(Key::G),
        'h' => Some(Key::H),
        'i' => Some(Key::I),
        'j' => Some(Key::J),
        'k' => Some(Key::K),
        'l' => Some(Key::L),
        'm' => Some(Key::M),
        'n' => Some(Key::N),
        'o' => Some(Key::O),
        'p' => Some(Key::P),
        'q' => Some(Key::Q),
        'r' => Some(Key::R),
        's' => Some(Key::S),
        't' => Some(Key::T),
        'u' => Some(Key::U),
        'v' => Some(Key::V),
        'w' => Some(Key::W),
        'x' => Some(Key::X),
        'y' => Some(Key::Y),
        'z' => Some(Key::Z),
        _ => None,
    }
}

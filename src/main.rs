use rand::{thread_rng, Rng};
use std::char;

const ASCII: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let name_range = get_name_len();
    let mut name = String::from("");

    for _i in 0..name_range {
        let random_ascii_number = get_character_number();
        let character = match digit_to_ascii(random_ascii_number) {
            Some(character) => character,
            None => ' ',
        };
        name.push_str(&character.to_string());
    }

    println!("{}", name);
}

fn get_name_len() -> i32 {
    let mut rng = thread_rng();
    let name_range = 5..11;
    let name_len = rng.gen_range(name_range);
    name_len
}

fn get_character_number() -> i32 {
    let mut rng = thread_rng();
    let ascii_range = 0..=25;
    let rnd = rng.gen_range(ascii_range);
    rnd
}

fn digit_to_ascii(digit: i32) -> Option<char> {
    let ascii_len = ASCII.len();

    if digit >= ascii_len as i32 {
        return None;
    }

    Some(ASCII[digit as usize])
}

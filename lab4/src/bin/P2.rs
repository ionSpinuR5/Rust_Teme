use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut result = String::new();

    for ch in content.chars() {
        if !ch.is_ascii() {
            println!("Eroare: textul contine un caracter non-ASCII.");
            return;
        }

        let rotated = if ch.is_ascii_lowercase() {
            (((ch as u8 - b'a' + 13) % 26) + b'a') as char
        } else if ch.is_ascii_uppercase() {
            (((ch as u8 - b'A' + 13) % 26) + b'A') as char
        } else {
            ch
        };

        result.push(rotated);
    }

    fs::write("output.txt", result).unwrap();
    println!("Textul criptat a fost scris in output.txt.");
}

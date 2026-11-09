use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        println!("Fisierul este gol.");
        return;
    }

    let mut longest_bytes = lines[0];
    let mut longest_chars = lines[0];

    for line in &lines {
        if line.len() > longest_bytes.len() {
            longest_bytes = line;
        }

        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }

    println!("Cea mai lunga linie in bytes: {}", longest_bytes);
    println!("Numar bytes: {}", longest_bytes.len());

    println!("Cea mai lunga linie in caractere: {}", longest_chars);
    println!("Numar caractere: {}", longest_chars.chars().count());
}

use std::fs;

fn main() {
    let content = fs::read_to_string("/etc/hosts").unwrap();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            println!("{} => {}", parts[1], parts[0]);
        }
    }
}

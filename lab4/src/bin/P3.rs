use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut result: Vec<&str> = Vec::new();

    for word in content.split_whitespace() {
        let replacement = match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        };

        result.push(replacement);
    }

    let output = result.join(" ");
    fs::write("output.txt", output).unwrap();

    println!("Fraza modificata a fost scrisa in output.txt.");
}

use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let content = fs::read_to_string("students.jsonl").unwrap();
    let mut students: Vec<Student> = Vec::new();

    for line in content.lines() {
        let student: Student = serde_json::from_str(line).unwrap();
        students.push(student);
    }

    if students.is_empty() {
        println!("Fisierul nu contine studenti.");
        return;
    }

    let mut youngest_index = 0;
    let mut oldest_index = 0;

    for i in 1..students.len() {
        if students[i].age < students[youngest_index].age {
            youngest_index = i;
        }

        if students[i].age > students[oldest_index].age {
            oldest_index = i;
        }
    }

    let youngest = &students[youngest_index];
    let oldest = &students[oldest_index];

    println!(
        "Cel mai tanar: {}, {}, {}",
        youngest.name, youngest.phone, youngest.age
    );

    println!(
        "Cel mai in varsta: {}, {}, {}",
        oldest.name, oldest.phone, oldest.age
    );
}

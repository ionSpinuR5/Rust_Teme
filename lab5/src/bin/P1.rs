use std::fs;

struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let content = fs::read_to_string("students.txt").unwrap();
    let mut students: Vec<Student> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();

        let student = Student {
            name: parts[0].to_string(),
            phone: parts[1].to_string(),
            age: parts[2].parse().unwrap(),
        };

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

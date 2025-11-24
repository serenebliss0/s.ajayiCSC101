use std::io::*;
use std::fs::*;

fn main() {
    struct Student {
        name: String,
        matric_number: String,
        department: String,
        level: String,
    }

    let mut students: Vec<Student> = Vec::new();

    println!("How many students data do you want to enter?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let number_of_students: u32 = input.trim().parse().expect("Enter a positive whole number");

    for i in 0..number_of_students {
        println!("Enter name of student {}", i + 1);
        let mut name_input = String::new();
        std::io::stdin().read_line(&mut name_input).expect("Failed to read line");
        let name = name_input.trim().to_string();

        println!("Enter matric number of student {}", i + 1);
        let mut matric_input = String::new();
        std::io::stdin().read_line(&mut matric_input).expect("Failed to read line");
        let matric_number = matric_input.trim().to_string();

        println!("Enter department of student {}", i + 1);
        let mut dept_input = String::new();
        std::io::stdin().read_line(&mut dept_input).expect("Failed to read line");
        let department = dept_input.trim().to_string();

        println!("Enter level of student {}", i + 1);
        let mut level_input = String::new();
        std::io::stdin().read_line(&mut level_input).expect("Failed to read line");
        let level = level_input.trim().to_string();

        // create the struct
        let student = Student {
            name,
            matric_number,
            department,
            level,
        };

        // push into the vector
        students.push(student);

        println!("Student {} added successfully!", i + 1);
    }

    // print all students
    for (i, s) in students.iter().enumerate() 
    {
        println!("Student {}: {} | {} | {} | {} level", i + 1, s.name, s.matric_number, s.department, s.level);
    }

    let mut file = OpenOptions::new().append(true).create(true).open("students.txt").expect("Failed to write to file");

    for s in &students 
    {
        let line = format!("{}\t{}\t{}\t{}\n", s.name, s.matric_number, s.department, s.level);
        file.write_all(line.as_bytes()).expect("Failed to write to file");
    }

    println!("All students have been added to students.txt!");

}

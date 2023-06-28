//main.rs

// Imports
use std::io;
use std::result;
use std::time::{SystemTime, UNIX_EPOCH};

// Enums
enum MovementType {
    Zumba,
    Yoga,
    Pilates,
    Ballet
}

// Structs
#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    dob: SystemTime,
    classes: Vec<MovementType>
}

// Traits
trait Academy {
    fn new_student(&mut self, name: String, dob: SystemTime) -> u32;
    fn add_class(&mut self, id: u32, class: MovementType);
}

impl Academy for MindfulMovementAcademy {
    fn new_student(&mut self, name: String, dob: SystemTime) -> u32 {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        let student = Student {
            id: timestamp.as_secs() as u32,
            name: name,
            dob: dob,
            classes: vec![]
        };

        self.students.push(student);
        return timestamp.as_secs() as u32;
    }

    fn add_class(&mut self, id: u32, class: MovementType) {
        for student in &mut self.students {
            if student.id == id {
                student.classes.push(class);
            }
        }
    }
}

// Structs
struct MindfulMovementAcademy {
    students: Vec<Student>,
}

// Functions
fn main() -> result::Result<(), io::Error> {
    let mut academy = MindfulMovementAcademy {
        students: vec![],
    };
    let student_id = academy.new_student(String::from("Jenny"), SystemTime::now());
    println!("created student with id {}", student_id);

    academy.add_class(student_id, MovementType::Zumba);
    academy.add_class(student_id, MovementType::Yoga);
    academy.add_class(student_id, MovementType::Ballet);

    println!("{:#?}", academy);
    Ok(())
}
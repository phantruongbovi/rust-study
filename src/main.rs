use std::{collections::HashMap, io};

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("pls try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<u8> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    let parse: Result<u8, _> = input.parse();
    match parse {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: u8,
}

#[derive(Debug)]
pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }

    fn view(&self) -> Vec<&Student> {
        self.class.values().collect()
    }

    fn del(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }

    fn edit(&mut self, name: &str, age: u8) -> bool {
        match self.class.get_mut(name) {
            Some(input) => {
                input.age = age;
                true
            }
            None => false,
        }
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

mod manager {
    use std::ops::Add;

    use crate::*;

    pub fn add_student(students: &mut Students) {
        print!("Enter your name: ");
        let name = match get_input() {
            Some(input) => input,
            _ => return,
        };
        print!("Enter your age: ");
        let age = match get_int() {
            Some(number) => number,
            _ => return,
        };
        let student = Student { name, age };
        students.add(student);
    }

    pub fn view(students: &Students) {
        for s in students.view() {
            println!("{:?}", s);
        }
    }

    pub fn delete(students: &mut Students) {
        self::view(students);
        print!("Choose name remove: ");
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        students.del(&input);
    }

    pub fn edit(students: &mut Students) {
        self::view(students);
        print!("Choose name want to edit: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        print!("Choose age want to edit: ");
        let age = match get_int() {
            Some(name) => name,
            None => return,
        };
        if students.edit(&name, age) {
            println!("Student has edit!");
        } else {
            println!("Edit fail!");
        }
    }
}

impl Manager {
    fn show() {
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        print!("Make choose: ");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your choice");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view(&students),

            Some(Manager::EditStudent) => manager::edit(&mut students),

            Some(Manager::DeleteStudent) => manager::delete(&mut students),
            _ => return,
        }
    }
}

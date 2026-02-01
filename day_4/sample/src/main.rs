use std::{fmt::Display, rc::Rc};

fn main() {
    let n = 9;
    let n1 = n;

    // Result
    let p = PersonType::Pupil(Person {
        age: 15,
        name: "A".to_string(),
    });
    let result = function_result(p);
    if let Ok(p) = result {
        dbg!(p);
    } else {
        println!("Err");
    }

    // Option
    let op: Option<i32> = None;
    // 1
    if let Some(v) = op {
        println!("Value: {}", v);
    } else {
        println!("Err");
    }
    // 2
    let value = match op {
        Some(v) => v,
        None => 0,
    };
    // 3
    let value2: i32 = op.unwrap_or(0);

    println!("value la {}", value2);
}

fn function_result(person: PersonType) -> Result<Person, String> {
    match person {
        PersonType::Pupil(person) => Err("Day la hoc sinh".to_string()),
        PersonType::Student(person) => Ok(person),
    }
}

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
}

impl Person {
    pub fn sing() {
        println!("I'm singing!")
    }

    fn new_func() -> () {
        todo!()
    }
}

impl Student for Person {
    fn go_to_uni() -> () {
        todo!()
    }
}

//--------

enum PersonType {
    Pupil(Person),
    Student(Person),
}

trait Student {
    fn go_to_uni() -> ();
}

fn smart_pointer() {
    let a: Box<i32> = Box::new(4);
    let b = a.clone();

    let r: Rc<String> = Rc::new("Hello".to_string());
    let r1: Rc<String> = r.clone();

    // Cell RefCell Arc Mutex
}

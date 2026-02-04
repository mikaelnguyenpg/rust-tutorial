use std::{
    cell::{Cell, Ref, RefCell},
    mem,
    rc::{Rc, Weak},
    sync::Arc,
    thread,
};

trait Person {
    fn handle(&self);
}

#[derive(Debug)]
struct Student {
    name: String,
    friend: Option<Box<Student>>,
}

impl Student {
    // constrctor
    fn new(name: String) -> Self {
        Student { name, friend: None }
    }
}

impl Person for Student {
    fn handle(&self) {
        println!("Student is study!");
    }
}

#[derive(Debug)]
struct Teacher {
    name: String,
    coworker: Option<Weak<Teacher>>,
}

impl Teacher {
    fn new(name: String) -> Self {
        Teacher {
            name: name,
            coworker: None,
        }
    }
}

fn describe_box() {
    // 1: su dung cho dyn
    // let p: Box<dyn Person>;
    // p = Box::new(Student {});
    // p.handle();

    // 2: khi co su xuat hien lap lai cua T trong mot field
}

fn describe_reference_counter() {
    let a = Rc::new(Student::new("A".to_string()));
    {
        let b = a.clone();

        println!("Counter a: {}", Rc::strong_count(&a));
        println!("Counter b: {}", Rc::strong_count(&b));
        println!("A: {:?}", a);

        mem::drop(a);
        println!("Counter b: {}", Rc::strong_count(&b));
        println!("B: {:?}", b);
    }
    // toan bo heap duoc giai phong
}

fn describe_atomic_reference_counter() {
    let a = Arc::new(Student::new("A".to_string()));

    let thread = thread::spawn(move || {
        let b = a.clone();

        println!("Counter a: {}", Arc::strong_count(&a));
        println!("Counter b: {}", Arc::strong_count(&b));
        println!("A: {:?}", a);

        mem::drop(a);
        println!("Counter b: {}", Arc::strong_count(&b));
        println!("B: {:?}", b);
    });

    thread.join().unwrap();

    // toan bo heap duoc giai phong
}

fn describe_cell() {
    let a = Cell::new(5);
    a.set(9);
}

fn describe_ref_cell() {
    let a = RefCell::new(Student::new("RefCell".to_string()));
    a.borrow_mut().name = "New Name".to_string();
    println!("A {:?}", a);
}

fn describe_weak_refence() {
    let a = RefCell::new(Teacher::new("A".to_string()));
    let b = RefCell::new(Teacher::new("B".to_string()));

    a.borrow_mut().coworker = Some(Rc::downgrade(&Rc::new(b.into_inner())));
    // b.borrow_mut().coworker = Some(Rc::downgrade(&Rc::new(a.into_inner())));

    println!("A: {:?}", a);
    // println!("B: {:?}", b);
}

fn main() {
    // describe_box();

    // describe_reference_counter();

    // describe_atomic_reference_counter();

    // describe_cell();

    // describe_ref_cell();

    describe_weak_refence();
}

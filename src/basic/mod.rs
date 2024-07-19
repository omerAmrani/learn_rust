
#![allow(unused)]

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8
}

impl Person {
    pub fn adult(&self) -> bool {
        return self.age > 18;
    }
}

pub fn print() {
    println!("Hello, world!");

    // Primitives
    let is_something: bool = true;
    let number: i32 = 3433;
    let mut block = "aa";
    println!("number {number} bool {is_something} str {block}");

    block = "AA";
    println!("{block} has changed");

    let name: String = String::from("omer");
    let age: u8 = 24;


    let omer = Person {name, age};
    println!("{:?} and {:?}" , omer.age, omer.name);

    if omer.age > 20 {
        println!("{} is bigger the 20", omer.age);
    }

    let mut count = 0;
    loop {
        println!("{} iteration", count + 1);

        count += 1;
        if count == 4 {
            println!("That's enough");
            break;
        }
    }

    println!("{} id adult? {}", omer.name, omer.adult());
}

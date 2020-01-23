// use std::cmp::Ordering;
use std::io;

enum TaskList {
    Resist,
    Show,
    Index,
}

enum Gender {
    Male,
    Female,
    Other,
}

struct Employee {
    name: String,
    gender: Gender,
    age: i32,
    salary: i32,
}

fn main() {
    println!("Hi, I'm employee management system. Let's select number.");

    loop {
        println!("1: resist");
        println!("2: show");
        println!("3: index");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let task: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", task);

        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         break;
        //     }
        // }
    }
}

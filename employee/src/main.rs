// use std::cmp::Ordering;
use std::io;

enum TaskKind {
    Resist,
    Show,
    Index,
}

enum Gender {
    Male,
    Female,
    Other,
}

struct TaskLog {
    kind: TaskKind,
}

struct Employee {
    name: String,
    // gender: Gender,
    // age: i32,
    // salary: i32,
}

fn main() {
    println!("Hi, I'm employee management system. Let's select number.");

    loop {
        println!("1: resist");
        println!("2: show");
        println!("3: index");
        println!("4: quit");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let task_num: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match task_num {
            1 => {
                println!("OK, let's resist employee.");

                println!("Please enter name.");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                // println!("Please enter gender.");
                // let mut gender = String::new();
                // io::stdin()
                //     .read_line(&mut n)
                //     .expect("Failed to read line");

                // println!("Please enter age.");
                // let mut age = String::new();
                // io::stdin()
                //     .read_line(&mut n)
                //     .expect("Failed to read line");
                // let age: u32 = match n.trim().parse() {
                //     Ok(num) => num,
                //     Err(_) => continue,
                // };

                // println!("Please enter salary.");
                // let mut salary = String::new();
                // io::stdin()
                //     .read_line(&mut n)
                //     .expect("Failed to read line");
                // let salary: u32 = match n.trim().parse() {
                //     Ok(num) => num,
                //     Err(_) => continue,
                // };

                let employee = Employee {
                    name: name,
                    // gender: Gender::Male,
                    // age: 23,
                    // salary: 900,
                };
            },
            2 => {
                println!("OK, let's show employee datase.");

                println!("Please tell me employee name you want to know.");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut n)
                    .expect("Failed to read line");
            },
            3 => {
                println!("OK, let's show employee list.");
            },
            4 => { break; }
            _ => println!("Please valid number.😠💢"),
        }

        println!("You selected: {}", task_num);

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

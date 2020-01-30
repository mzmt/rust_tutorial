// use std::cmp::Ordering;
use std::io;

enum TaskKind {
    Resist,
    Show,
    Index,
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Other,
}

struct TaskLog {
    id: i32,
    kind: TaskKind,
}

struct Employee {
    name: String,
    gender: Gender,
    // age: i32,
    // salary: i32,
}

impl Employee {
    fn find<'a>(list: &'a Vec<Employee>, name: &str) -> Option<&'a Employee> {
        for employee in list {
            if employee.name == name {
                return Some(employee)
            }
        }
        None
    }
}

fn main() {
    // オンメモリ上に従業員のデータを保存する
    let mut employee_list =  vec![];

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

                println!("Please select gender.");
                println!("1: male, 2: female, 3: other.");
                let mut gender_num = String::new();
                io::stdin()
                    .read_line(&mut gender_num)
                    .expect("Failed to read line");
                let num: u32 = match gender_num.trim().parse() {
                    Ok(g) => g,
                    Err(_) => {
                        println!("Error");
                        continue;
                    },
                };

                let mut gender: Gender = Gender::Male;
                match num {
                    1 => {
                        gender = Gender::Male;
                    },
                    2 => {
                        gender = Gender::Female;
                    },
                    3 => {
                        gender = Gender::Other;
                    },
                    _ => ()
                }


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
                    gender: gender,
                    // age: 23,
                    // salary: 900,
                };

                employee_list.push(employee);
                println!("<success>");
            },
            2 => {
                println!("OK, let's show employee datase.");
                println!("Please tell me employee name you want to know.");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let employee: Option<&Employee> = Employee::find(&employee_list, &name);

                match employee {
                    Some(e) => println!("employee name is {}", e.name),
                    None => println!("Nothing found."),
                }
                // Someが返って来た時だけ書くならこう
                // if let Some(e) = employee {
                //     println!("employee name is {}", e.name);
                // }
            },
            3 => {
                println!("OK, let's show employee list.");

                for i in &employee_list {
                    println!("name: {}", i.name);
                }
            },
            4 => { break; }
            _ => println!("Please valid number.😠💢"),
        }

        println!("You selected: {}", task_num);
    }
}

mod Task;
mod file_utils;

use crate::Task::TaskList;
use crate::file_utils::{read_file, save_list};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    const FILENAME: &str = "tasks.dat";

    let task_list: TaskList = read_file(FILENAME);
    let mut buffer: String = String::new();
    let mut option: i32 = 0;

    loop {
        clear_screen();
        print_tasks(&task_list);
        print_options();

        io::stdin().read_line(&mut buffer).unwrap();
        option = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, input numbers only.");
                continue;
            }
        };

        match option {
            1 => {}

            2 => {
                save_list(&task_list, FILENAME);
            }

            3 => {}

            4 => {
                save_list(&task_list, FILENAME);
                return;
            }

            _ => {
                println!("Not a valid option.");
            }
        }
    }
}

fn print_options() {
    println!("Options:");
    println!("1. Create new task");
    println!("2. Save List");
    println!("3. Mark task done");
    println!("4. Save and exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // Clears the screen and moves cursor to top-left
}

fn print_tasks(task_list: &TaskList) {
    for task in task_list.iter() {
        println!("=====================");
        println!("{}: {}", task.title, task.description);
        println!("Status: {}", task.status_text());
        println!("=====================");
    }
}

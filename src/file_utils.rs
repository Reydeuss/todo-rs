use std::fs::{File, exists, rename};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;

use crate::task::{Task, TaskList};

fn open_file(filename: &str) -> io::Result<File> {
    let filepath = Path::new(filename);
    let display = filepath.display();

    if !filepath.exists() {
        let mut buffer = String::new();

        println!("The save file is not found. Create new? [Y/n]");

        io::stdin().read_line(&mut buffer)?;

        let input = buffer.trim();

        match input {
            "Y" | "y" | "" => return create_file(filename),
            _ => {
                println!("Invalid input. Aborting.");
                process::exit(1);
            }
        }
    }

    println!("Opening {display}...");
    File::open(filepath)
}

fn create_file(filename: &str) -> io::Result<File> {
    File::create(filename)
}

/*
* This function parses a line of text and initialises a new Task
* item with the proper fields.
*
* The parser uses a CSV-style pattern.
* Format: TITLE,DESCRIPTION,STATUS[true/false].
*/
fn parse_task(text: String) -> Task {
    let fields: Vec<&str> = text.split(',').collect();

    if fields.len() != 3 {
        panic!("there is a problem with the file format. panicking now");
    }

    let title: &str = fields[0].trim();
    let description: &str = fields[1].trim();
    let status: bool = match fields[2] {
        "true" => true,
        "false" => false,
        _ => panic!("there is a problem with the file format. panicking now"),
    };

    Task::new(title, description, status)
}

pub fn read_file(filename: &str) -> io::Result<TaskList> {
    let file = open_file(filename)?;
    let reader = BufReader::new(file);
    let mut task_list: TaskList = Vec::new();

    for line in reader.lines() {
        let task = parse_task(line?);
        task_list.push(task);
    }

    Ok(task_list)
}

pub fn save_list(task_list: &TaskList, filename: &str) {
    let filepath: &Path = Path::new(filename);
    let tmp_path = filepath.with_extension("tmp");
    let mut tmp_file: File = match File::create(&tmp_path) {
        Ok(f) => f,
        Err(why) => panic!(
            "Error: unable to create file. Aborting changes.\nReason: {}",
            why
        ),
    };

    for task in task_list.iter() {
        match write!(tmp_file, "{}\n", task.stringify()) {
            Ok(_) => (),
            Err(why) => panic!(
                "Unable to write to temporary file. Aborting changes.\nReason: {}",
                why
            ),
        }
    }

    match tmp_file.sync_all() {
        Ok(ok) => ok,
        Err(why) => panic!("Error: unable to save changes. Aborting.\nReason: {}", why),
    }

    match rename(&tmp_path, filepath) {
        Ok(_) => (),
        Err(why) => panic!("Error: unable commit atomic changes. Reason: {}", why),
    }
}

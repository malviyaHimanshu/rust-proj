use std::fs::File;
use std::{env, fs::OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

// for adding a new task which will save to tasks.txt
fn add_task(task: &str) {
    let tasks = get_tasks().unwrap_or_else(|_| Vec::new());
    let next_id = tasks.len() + 1;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tasks.txt")
        .expect("Cannot open file");

    writeln!(file, "{next_id}: {task}").expect("Cannot write to file");
}

fn get_tasks() -> io::Result<Vec<String>> {
    let file = File::open("tasks.txt")?;
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();
    for line in reader.lines() {
        let task = line?;
        tasks.push(task);
    }

    Ok(tasks)
}

fn remove_task(task_id: i32) -> io::Result<()> {
    let tasks = get_tasks()?;
    if task_id==0 || task_id > tasks.len() as i32 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid task number."));
    }
    let filtered_tasks: Vec<String> = tasks
        .into_iter()
        .enumerate()
        .filter_map(|(index, task)| {
            if index as i32 + 1 == task_id {
                None
            } else {
                Some(task)
            }
        })
        .collect();

        let mut file = File::create("tasks.txt")?;
        for (i, task) in filtered_tasks.iter().enumerate() {
            writeln!(file, "{}: {}", i+1, task.split(": ").skip(1).collect::<Vec<_>>().join(": "))?;
        }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: todo <command> [arguments]");
        return;
    }
    
    let command = &args[2];

    match command.as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Usage: todo add <task>");
                return;
            }
            let task = &args[3];
            add_task(&task.as_str());
            println!("Added task: {}", task);
        },
        "list" => {
            // since get_tasks return Result<T, E>, we handle it like below.
            match get_tasks() {
                Ok(tasks) => {
                    if tasks.is_empty() {
                        println!("No tasks present.");
                    } else {
                        for task in tasks {
                            println!("{task}");
                        }
                    }
                },
                Err(e) => {
                    println!("Error listing tasks: {e}");
                }
            }
        },
        "remove" => {
            if args.len() < 4 {
                println!("Usage: todo remove <task_id>");
                return;
            }
            let task_id: i32 = match args[3].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please provide a valid task_id");
                    return;
                }
            };
            
            match remove_task(task_id) {
                Ok(()) => {
                    println!("Removed task: {task_id}");
                },
                Err(e) => {
                    println!("Error removing task: {e}");
                }
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: todo <add|list|remove> [arguments]");
        }
    }
}
#![allow(unused_parens)]

pub mod task;
pub use task::task::Task;
pub use task::task_list::TaskList;

use chrono::prelude::Local;
use std::{io, usize};

fn main() {
    println!("Hello, Welcome to Ducky's task Manager");
    let mut task_list = TaskList::load();
    

    loop {
        println!("Commands:");
        println!("- add <title> <description> <due_date_time>");
        println!("- view");
        println!("- complete <task_index>");
        println!("- filter <completed | upcoming>\\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts[0] {
            "add" => match parts.len() {
                4 => {
                    task_list.add(
                        Task::new(
                            parts[1].to_string(),
                            parts[2].to_string(),
                            chrono::NaiveDateTime::parse_from_str(parts[3], "%Y-%m-%dT%H:%M:%S").unwrap()
                        )   
                    );
                    task_list.save().unwrap();
                },
                _ => println!("Invalid Add command parts"),
            }
            "view" => match parts.len() {
                1 => {
                    for (index, task) in task_list.get_all().iter().enumerate() {
                        println!(
                            "{}. {} {} (Due: {})",
                            index,
                            task.title,
                            task.get_completed(),
                            task.due_date_time
                        );
                    }
                }
                2 => match task_list.get(parts[1].parse::<usize>().unwrap()) {
                    None => {
                        println!("Task Not Found!")
                    }
                    Some(task) => {
                        println!(
                            "{}. {} {} (Due: {})",
                            parts[1],
                            task.title,
                            task.get_completed(),
                            task.due_date_time
                        );
                    }
                },
                _ => println!("Invalid view command parts"),
            },
            "complete" => match parts.len() {
                2 => match task_list.complete_task(parts[1].parse::<usize>().unwrap()) {
                    Err(message) => {
                        println!("Error! {}", message);
                    }
                    Ok(message) => {
                        println!("Sucess! {}", message);
                        let _ = task_list.save();
                    }
                },
                _ => {
                    println!("Invalid complete command");
                }
            },
            "filter" => {
                if (parts.len() == 2) {
                    match parts[1] {
                        "completed" => {
                            for (index, task) in task_list
                                .get_all()
                                .iter()
                                .enumerate()
                                .filter(|t| t.1.completed)
                            {
                                println!("{}. {} [Complete]", index + 1, task.title);
                            }
                        }
                        "upcoming" => {
                            let today = Local::now().naive_local();
                            for (index, task) in task_list
                                .get_all()
                                .iter()
                                .enumerate()
                                .filter(|t| t.1.due_date_time >= today)
                            {
                                println!(
                                    "{}. {} {} (Due: {})",
                                    index + 1,
                                    task.title,
                                    task.get_completed(),
                                    task.due_date_time
                                );
                            }
                        }
                        _ => println!("Invalid filter option."),
                    }
                } else {
                    println!("Invalid input. Use: filter <completed | upcoming>");
                }
            }
            _ => println!("Invalid command."),
        }
    }
}

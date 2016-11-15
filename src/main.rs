#[macro_use]
extern crate clap;
extern crate rustc_serialize;
extern crate term;

use std::path::PathBuf;

mod cli;
mod todo;
use self::todo::{Todo, TodoList, Priority};

fn main() {
	let matches = cli::build_cli().get_matches();
    let path = match matches.value_of("file") {
        Some(path) => PathBuf::from(String::from(path)),
        None => TodoList::get_default_path().unwrap(),
    };

    let list = match TodoList::from_path(path.as_path()) {
        Ok(l) => l,
        Err(_) => TodoList::new(),
    };

    let result = match matches.subcommand() {
        ("list", Some(_)) => {
            list.show();
            None
        },
        ("add", Some(m)) => {
            let task = Todo {
                id: list.get_next_id(),
                name: String::from(m.value_of("name").unwrap()),
                details: String::from(m.value_of("details").or(Some("")).unwrap()),
                done: false,
                priority: Priority::from_matches(m),
            };
            println!("Task added!");
            task.show();
            println!("");
            Some(list.add_todo(task))
        },
        (action, Some(m)) => {
            let id = value_t_or_exit!(m, "task", u64);
            let (index, task) = match list.find_by_id(id) {
                Some(d) => d,
                None => {
                    println!("Task #{} not found", id);
                    return
                }
            };
            match action {
                "show" => {
                    task.show();
                    None
                },
                "remove" => {
                    println!("Task #{} removed", id);
                    Some(list.remove_todo(index))
                },
                "done" => {
                    let done = task.done(true);
                    done.show();
                    Some(list.replace_todo(index, done))
                },
                "todo" => {
                    let todo = task.done(false);
                    todo.show();
                    Some(list.replace_todo(index, todo))
                },
                _ => None,
            }
        },
        _ => None,
    };

    match result {
        Some(l) => l.save_to(path.as_path()),
        None => Ok(()),
    }.unwrap()
}

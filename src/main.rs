mod models;
mod storage;

use models::Task;
use storage::{load_tasks, save_tasks};
use std::env;
use std::process;

fn usage() -> &'static str {
    "Usage: todo <command> [args]\n\
     Commands:\n\
       add <name> [description]   Add a new task\n\
       list                      List all tasks\n\
       show <id>                 Show a single task\n\
       done <id>                 Mark a task as done\n\
       remove <id>               Remove a task\n\
       help                      Show this help"
}

fn next_id(tasks: &[Task]) -> u64 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "help" {
        println!("{}", usage());
        return;
    }

    let mut tasks = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: failed to load tasks: {}", e);
            process::exit(1);
        }
    };

    let cmd = &args[0];
    let rest = &args[1..];

    match cmd.as_str() {
        "add" => {
            if rest.is_empty() {
                eprintln!("error: 'add' requires a task name");
                println!("\n{}", usage());
                process::exit(1);
            }
            let name = rest[0].to_string();
            let description = if rest.len() > 1 {
                rest[1].to_string()
            } else {
                String::new()
            };
            let id = next_id(&tasks);
            let task = Task::new(id, name, description);
            tasks.push(task);
            if let Err(e) = save_tasks(&tasks) {
                eprintln!("error: failed to save tasks: {}", e);
                process::exit(1);
            }
            println!("added task #{}: {}", id, tasks.last().unwrap().name);
        }
        "list" => {
            if tasks.is_empty() {
                println!("no tasks yet. Add one with: todo add \"My task\"");
                return;
            }
            println!("{:<5} {:<6} {:<12} {}", "ID", "DONE", "CREATED", "NAME");
            for t in &tasks {
                println!(
                    "{:<5} {:<6} {:<12} {}",
                    t.id,
                    if t.done { "yes" } else { "no" },
                    t.created_at,
                    t.name
                );
            }
        }
        "show" => {
            if rest.is_empty() {
                eprintln!("error: 'show' requires a task id");
                process::exit(1);
            }
            let id: u64 = match rest[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("error: invalid id '{}'", rest[0]);
                    process::exit(1);
                }
            };
            match tasks.iter().find(|t| t.id == id) {
                Some(t) => {
                    println!("id:         {}", t.id);
                    println!("name:       {}", t.name);
                    println!("description: {}", t.description);
                    println!("created:    {}", t.created_at);
                    println!("done:       {}", if t.done { "yes" } else { "no" });
                }
                None => {
                    eprintln!("error: task #{} not found", id);
                    process::exit(1);
                }
            }
        }
        "done" => {
            if rest.is_empty() {
                eprintln!("error: 'done' requires a task id");
                process::exit(1);
            }
            let id: u64 = match rest[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("error: invalid id '{}'", rest[0]);
                    process::exit(1);
                }
            };
            let mut found = false;
            for t in &mut tasks {
                if t.id == id {
                    if t.done {
                        println!("task #{} is already done", id);
                    } else {
                        t.done = true;
                        println!("marked task #{} as done", id);
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                eprintln!("error: task #{} not found", id);
                process::exit(1);
            }
            if let Err(e) = save_tasks(&tasks) {
                eprintln!("error: failed to save tasks: {}", e);
                process::exit(1);
            }
        }
        "remove" => {
            if rest.is_empty() {
                eprintln!("error: 'remove' requires a task id");
                process::exit(1);
            }
            let id: u64 = match rest[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("error: invalid id '{}'", rest[0]);
                    process::exit(1);
                }
            };
            let len_before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() == len_before {
                eprintln!("error: task #{} not found", id);
                process::exit(1);
            }
            if let Err(e) = save_tasks(&tasks) {
                eprintln!("error: failed to save tasks: {}", e);
                process::exit(1);
            }
            println!("removed task #{}", id);
        }
        other => {
            eprintln!("error: unknown command '{}'", other);
            println!("\n{}", usage());
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use storage::{load_tasks_at, save_tasks_at};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = env::temp_dir().join(format!("todo-cli-test-{}", nanos));
        fs::create_dir_all(&dir).unwrap();
        dir.join("tasks.json")
    }

    #[test]
    fn test_next_id_empty() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn test_next_id_skips_gaps() {
        let tasks = vec![
            Task { id: 1, name: "a".into(), description: String::new(), created_at: String::new(), done: false },
            Task { id: 5, name: "b".into(), description: String::new(), created_at: String::new(), done: true },
        ];
        assert_eq!(next_id(&tasks), 6);
    }
}
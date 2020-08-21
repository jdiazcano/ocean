use build_system::BuildSystem;
use build_system::beans::UserConfiguration;
use std::process::Command;
use colored::*;

pub struct Rust;

impl BuildSystem for Rust {
    fn build(&self, folder: &str, config: &UserConfiguration) -> i32 {
        execute(&["build"], folder).unwrap()
    }

    fn release(&self, folder: &str, config: &UserConfiguration) -> i32 {
        execute(&["build", "--release"], folder).unwrap()
    }

    fn clean(&self, folder: &str, config: &UserConfiguration) -> i32 {
        execute(&["clean"], folder).unwrap()
    }

    fn publish(&self, folder: &str, config: &UserConfiguration, local: bool) -> i32 {
        unimplemented!()
    }
}

fn execute(actions: &[&str], path: &str) -> Option<i32> {
    let mut command = Command::new("cargo");
    for action in actions {
        command.arg(action);
    }
    command.current_dir(path);
    println!("Executing command: {:?}", command);

    let status = command.status().ok().expect("Failed");
    match status.code() {
        Some(0) => {
            println!("{}", "Finished".green());
        }
        _ => {
            let formatted = format!("{}{:?}{}", "Failed executing '", actions, "'");
            println!("{}", formatted.red())
        }
    }

    return status.code()
}
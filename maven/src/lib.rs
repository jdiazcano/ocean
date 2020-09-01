use build_system::BuildSystem;
use build_system::beans::UserConfiguration;
use std::process::Command;
use colored::*;

pub struct Maven;

impl BuildSystem for Maven {
    fn build(&self, folder: &str, _config: &UserConfiguration) -> i32 {
        execute("build", folder).unwrap()
    }

    fn release(&self, folder: &str, _config: &UserConfiguration) -> i32 {
        execute("release", folder).unwrap()
    }

    fn clean(&self, folder: &str, _config: &UserConfiguration) -> i32 {
        execute("clean", folder).unwrap()
    }

    fn publish(&self, folder: &str, config: &UserConfiguration, local: bool) -> i32 {
        unimplemented!()
    }
}

fn execute(action: &str, path: &str) -> Option<i32> {
    let mut command = Command::new("mvn");
    command.arg(action);
    command.current_dir(path);

    let status = command.status().ok().expect("Failed");
    match status.code() {
        Some(0) => {
            println!("{}", "Finished".green());
        }
        _ => {
            let formatted = format!("{}{}{}", "Failed executing '", action, "'");
            println!("{}", formatted.red())
        }
    }

    return status.code()
}
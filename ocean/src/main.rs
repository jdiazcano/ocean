mod commands;
mod utils;

use std::{fs, env};
use std::process::{exit, Command};
use crate::commands::{BuildCommand, ReleaseCommand, CleanCommand, CdpCommand, WsCommand, SyncwsCommand, ExecutableCommand, ProjectCommand};
use build_system::beans::UserConfiguration;
use clap::{App, ArgMatches, AppSettings};
use build_system::BuildSystem;
use gradle_build_system::Gradle;
use crate::utils::{find_multiple_repos, find_path};
use colored::*;
use rust_build_system::Rust;
use std::path::Path;
use maven_build_system::Maven;

fn subcommands() -> Vec<App<'static>> {
    let subcommands = vec![
        BuildCommand::command(),
        CleanCommand::command(),
        ReleaseCommand::command(),
        ProjectCommand::command(),
        WsCommand::command(),
        // SyncwsCommand::command(),
        CdpCommand::command()
    ];
    return subcommands;
}

fn main() {
    let filename = format!("{}.json", whoami::username());
    let path = dirs::home_dir().unwrap().join(".ocean").join(filename);
    let full_path = path.canonicalize().unwrap();
    if !full_path.exists() {
        println!("Workspace file does not exist. (~/.ocean/$USERNAME.json)");
        return;
    }

    let file_contents = fs::read_to_string(full_path).expect("Unable to read file");
    let config: UserConfiguration = serde_json::from_str(&file_contents).unwrap();

    let matches = App::new("ocean")
        .version("1.0")
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommands(subcommands())
        .get_matches();

    match matches.subcommand() {
        ("build", Some(matches)) => execute_everywhere(&config, matches, |folder, config, _| find_build_system(folder).build(folder, config)),
        ("release", Some(matches)) => execute_everywhere(&config, matches, |folder, config, _| find_build_system(folder).release(folder, config)),
        ("clean", Some(matches)) => execute_everywhere(&config, matches, |folder, config, _| find_build_system(folder).clean(folder, config)),
        ("publish", Some(matches)) => execute_everywhere(&config, matches, |folder, config, _| find_build_system(folder).publish(folder, config, true)),
        ("cdp", Some(matches)) => CdpCommand::exec(&config, matches),
        ("project", Some(matches)) => ProjectCommand.handle(config, matches),
        ("ws", Some(matches)) => WsCommand::exec(),
        ("syncws", Some(matches)) => SyncwsCommand.handle(config, matches),
        (external, Some(matches)) => {
            // TODO Change this to loop through $PATH instead of SHELL
            let status = Command::new(env!("SHELL"))
                .arg("-c")
                .arg(format!("ocean-{}", external))
                .status();

            match status.unwrap().code().unwrap() {
                127 => {
                    println!("Unknown subcommand: {}", external);
                    exit(1)
                }
                code @ _ => exit(code)
            }
        },
        _ => {exit(-1)}
    };
}

fn execute_everywhere(config: &UserConfiguration, matches: &ArgMatches, function: impl Fn(&str, &UserConfiguration, &ArgMatches) -> i32) {
    let alias = matches.value_of("alias");

    if alias.is_none() {
        function(".", config, matches);
    }

    let repositories = find_multiple_repos(&config, alias.unwrap())
        .expect("No repositories were found");

    for repository in repositories {
        println!("Executing command for repository {}", repository.alias);
        let path = find_path(config, alias.unwrap());
        println!("Executing build for: {}", repository.alias.green());
        println!("Folder: {}", path);
        function(&path, config, matches);
    }
}

fn find_build_system(folder: &str) -> Box<dyn BuildSystem> {
    let path = Path::new(folder);
    return if path.join("Cargo.toml").exists() {
        Box::new(Rust)
    } else if path.join("build.gradle.kts").exists() {
        Box::new(Gradle)
    } else if path.join("build.gradle").exists() {
        Box::new(Gradle)
    } else if path.join("pom.xml").exists() {
        Box::new(Maven)
    } else {
        panic!("Unknown build system.")
    }
}

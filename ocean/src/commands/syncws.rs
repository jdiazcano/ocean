use crate::commands::{ExecutableCommand, SyncwsCommand};
use clap::{ArgMatches, App};
use build_system::beans::UserConfiguration;
use std::path::Path;
use std::process::Command;
use std::{fs, env};

impl SyncwsCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("syncws");
    }
}

impl ExecutableCommand for SyncwsCommand {
    fn handle(&self, config: UserConfiguration, matches: &ArgMatches) {
        let root = env::var("WORKSPACE_ROOT").expect("Variable not set");
        let root = Path::new(&root);

        for project in config.projects {
            println!("Checking {}", project.folder);
            let project_folder = root.join(project.folder);
            fs::create_dir_all(&project_folder);
            for repo in project.repositories {
                let repo_folder = project_folder.join(&repo.folder);
                if !repo_folder.exists() {
                    println!("Cloning {} ({})", repo.name, repo_folder.to_str().unwrap());
                    let mut command = Command::new("git");
                    command.arg("-C");
                    command.arg(&project_folder);
                    command.arg("clone");
                    command.arg("--recurse-submodules");
                    command.arg(repo.url);
                    command.arg(&repo.folder);
                    command.status();
                }
            }
        }
    }
}
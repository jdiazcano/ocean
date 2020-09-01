use crate::commands::{ProjectCommand, ExecutableCommand};
use clap::{App, Arg, ArgMatches};
use build_system::beans::UserConfiguration;
use std::process::Command;
use crate::utils::{find_path};
use std::env;

impl ProjectCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("project")
            .arg(Arg::new("alias"))
            .arg("-l, --list")
            .arg("-p, --print")
            .arg("-e, --executables");
    }
}

impl ExecutableCommand for ProjectCommand {
    fn handle(&self, config: UserConfiguration, matches: &ArgMatches) {
        if matches.is_present("list") {
            let projects: Vec<String> = config.projects.into_iter()
                .flat_map(|it| it.repositories)
                .map(|it| it.alias)
                .collect();
            let groups: Vec<String> = config.groups.unwrap_or(vec![]).into_iter()
                .map(|it| it.name)
                .collect();
            let all = [&projects[..], &groups[..]].concat();
            println!("{}", all.join(" "));
            return;
        }

        ProjectCommand::handle_command(&config, matches);
    }
}

impl ProjectCommand {
    fn handle_command(config: &UserConfiguration, matches: &ArgMatches) {
        let alias = matches.value_of("alias");
        match alias {
            Some(alias) => {
                if matches.is_present("print") {
                    let folder = find_path(&config, alias);
                    println!("{}", folder)
                } else if matches.is_present("executables") {
                    println!("Still not implemented!")
                } else {
                    let folder = find_path(&config, alias);
                    let mut command = Command::new("idea");
                    command.arg(folder);
                    command.spawn();
                }
            }
            None => {
                if matches.is_present("print") {
                    let current_dir = env::current_dir().unwrap();
                    let folder = current_dir.to_str().unwrap();
                    println!("{}", folder)
                } else if matches.is_present("executables") {
                    println!("Still not implemented!")
                } else {
                    let mut command = Command::new("idea");
                    command.arg(".");
                    command.spawn();
                }
            }
        }
    }
}

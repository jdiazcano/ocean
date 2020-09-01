use crate::commands::{ExecutableCommand, SyncwsCommand};
use clap::{ArgMatches, App};
use build_system::beans::UserConfiguration;

impl SyncwsCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("syncws");
    }
}

impl ExecutableCommand for SyncwsCommand {
    fn handle(&self, config: UserConfiguration, matches: &ArgMatches) {

    }
}
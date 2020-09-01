mod syncws;
mod ws;
mod project;

use clap::{ArgMatches, App, Arg};
use build_system::beans::UserConfiguration;
use crate::utils::{find_path};

pub trait ExecutableCommand {
    fn handle(&self, config: UserConfiguration, matches: &ArgMatches);
}

pub struct BuildCommand;
impl BuildCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("build").arg(Arg::new("alias"));
    }
}

pub struct CleanCommand;
impl CleanCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("clean").arg(Arg::new("alias").default_value("."));
    }
}

pub(crate) struct ReleaseCommand;
impl ReleaseCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("release").arg(Arg::new("alias").default_value("."));
    }
}

pub struct CdpCommand;
impl CdpCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("cdp").arg(Arg::new("alias"));
    }

    pub fn exec(config: &UserConfiguration, matches: &ArgMatches) {
        let alias = matches.value_of("alias").expect("An alias must be provided");

        let repository = find_path(config, alias);

        println!("{}", repository);
    }
}

pub struct WsCommand;
pub struct ProjectCommand;
pub struct SyncwsCommand;
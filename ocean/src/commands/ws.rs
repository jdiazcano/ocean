use crate::commands::WsCommand;
use clap::App;
use std::env;

impl WsCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("ws");
    }

    pub fn exec() {
        println!("{}", env::var("WORKSPACE_ROOT").expect("Workspace root not set"));
    }
}
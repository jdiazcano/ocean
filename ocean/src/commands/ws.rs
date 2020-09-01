use crate::commands::WsCommand;
use clap::App;

impl WsCommand {
    #[inline]
    pub fn command() -> App<'static> {
        return App::new("ws");
    }

    pub fn exec() {
        println!("{}", option_env!("WORKSPACE_ROOT").unwrap());
    }
}
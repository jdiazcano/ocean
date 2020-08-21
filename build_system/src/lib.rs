use crate::beans::UserConfiguration;

pub mod beans;

pub trait BuildSystem {
    fn build(&self, folder: &str, config: &UserConfiguration) -> i32;
    fn release(&self, folder: &str, config: &UserConfiguration) -> i32;
    fn clean(&self, folder: &str, config: &UserConfiguration) -> i32;
    fn publish(&self, folder: &str, config: &UserConfiguration, local: bool) -> i32;
}
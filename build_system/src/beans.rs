use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserConfiguration {
    pub projects: Vec<Project>,
    pub groups: Option<Vec<RepositoryGroup>>
}

#[derive(Serialize, Deserialize)]
pub struct RepositoryGroup {
    pub name: String,
    pub repositories: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub folder: String,
    pub repositories: Vec<Repository>
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub alias: String,
    pub url: String,
    pub folder: String
}

impl PartialEq for Repository {
    fn eq(&self, other: &Self) -> bool {
        self.alias == other.alias
    }
}
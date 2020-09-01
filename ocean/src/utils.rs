use build_system::beans::{UserConfiguration, Project, Repository};
use std::path::Path;

pub fn find_project<'c>(config: &'c UserConfiguration, alias: &str) -> &'c Project {
    let option = config.projects.iter()
        .find(|project| project.repositories.iter().any(|repo| repo.alias == alias));
    return option.expect("");
}

pub fn find_repository<'c>(config: &'c UserConfiguration, alias: &str) -> Option<&'c Repository> {
    return config.projects.iter()
        .flat_map(|project| project.repositories.iter())
        .find(|item| item.alias == alias)
}

pub fn find_path(config: &UserConfiguration, alias: &str) -> String {
    let root_path = option_env!("WORKSPACE_ROOT").unwrap();
    let project = find_project(config, alias);
    let repository = find_repository(config, alias).expect("Repository not found");

    let path = Path::new(root_path).join(&project.folder).join(&repository.folder);
    let str_path = path.to_str().unwrap();

    str_path.to_owned()
}

pub fn find_multiple_repos<'c>(config: &'c UserConfiguration, alias: &str) -> Option<Vec<&'c Repository>> {
    let vec1 = vec![];
    let multiple_repos = config.groups.as_ref().unwrap_or(&vec1).into_iter()
        .find(|group| group.name == alias)
        .and_then(|group| Some(group.repositories.iter()
            .map(|alias| find_repository(config, alias))
        ))
        .ok_or_else(|| Some::<Vec<&Repository>>(vec![]));
    let repo = find_repository(config, alias)
        .and_then(|repo| Some(vec![repo]))
        .ok_or_else(|| multiple_repos);

    return repo.ok();
}
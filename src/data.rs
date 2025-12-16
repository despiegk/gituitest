use crate::models::*;
use std::fs;
use std::path::Path;

pub fn load_repositories() -> Vec<Repository> {
    let path = Path::new("data/repos.toml");
    let content = fs::read_to_string(path).expect("Failed to read repos.toml");
    let data: RepoData = toml::from_str(&content).expect("Failed to parse repos.toml");
    data.repositories
}

pub fn load_files(repo_path: &str) -> Vec<FileEntry> {
    let path = Path::new("data/files.toml");
    let content = fs::read_to_string(path).expect("Failed to read files.toml");
    let data: FilesData = toml::from_str(&content).expect("Failed to parse files.toml");

    data.files
        .into_iter()
        .filter(|f| f.path.starts_with(repo_path) || repo_path.is_empty())
        .collect()
}

pub fn load_file_content(file_path: &str) -> Option<FileContent> {
    let path = Path::new("data/files.toml");
    let content = fs::read_to_string(path).expect("Failed to read files.toml");
    let data: FilesData = toml::from_str(&content).expect("Failed to parse files.toml");

    data.file_contents
        .into_iter()
        .find(|f| f.path == file_path)
}

pub fn load_commits() -> Vec<Commit> {
    let path = Path::new("data/commits.toml");
    let content = fs::read_to_string(path).expect("Failed to read commits.toml");
    let data: CommitsData = toml::from_str(&content).expect("Failed to parse commits.toml");
    data.commits
}

pub fn load_branches() -> Vec<Branch> {
    let path = Path::new("data/branches.toml");
    let content = fs::read_to_string(path).expect("Failed to read branches.toml");
    let data: BranchesData = toml::from_str(&content).expect("Failed to parse branches.toml");
    data.branches
}

pub fn get_repository(owner: &str, name: &str) -> Option<Repository> {
    load_repositories()
        .into_iter()
        .find(|r| r.owner == owner && r.name == name)
}

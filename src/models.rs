use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub is_mirror: bool,
    #[serde(default)]
    pub mirror_url: String,
    #[serde(default)]
    pub synced_ago: String,
    pub stars: u32,
    pub forks: u32,
    pub watchers: u32,
    pub commits: u32,
    pub branches: u32,
    pub tags: u32,
    pub size: String,
    pub default_branch: String,
    pub topics: Vec<String>,
    pub ssh_url: String,
    pub https_url: String,
    #[serde(default)]
    pub website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub commit_message: String,
    pub commit_time: String,
    #[serde(default)]
    pub issue_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub name: String,
    pub content: String,
    pub language: String,
    pub lines: u32,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    #[serde(default)]
    pub author_initial: String,
    pub message: String,
    pub time: String,
    pub is_signed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoData {
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesData {
    pub files: Vec<FileEntry>,
    pub file_contents: Vec<FileContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitsData {
    pub commits: Vec<Commit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchesData {
    pub branches: Vec<Branch>,
}

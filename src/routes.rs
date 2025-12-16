use actix_web::{web, HttpResponse, Result};
use askama::Template;

use crate::data::{get_repository, load_branches, load_commits, load_file_content, load_files};
use crate::models::*;

// DaisyUI Templates
#[derive(Template)]
#[template(path = "daisyui/repo.html")]
pub struct DaisyRepoTemplate {
    pub repo: Repository,
    pub files: Vec<FileEntry>,
    pub commits: Vec<Commit>,
    pub branches: Vec<Branch>,
    pub current_branch: String,
    pub current_path: String,
}

#[derive(Template)]
#[template(path = "daisyui/file.html")]
pub struct DaisyFileTemplate {
    pub repo: Repository,
    pub file: FileContent,
    pub commits: Vec<Commit>,
    pub branches: Vec<Branch>,
    pub current_branch: String,
}

// Tailwind Templates
#[derive(Template)]
#[template(path = "tailwind/repo.html")]
pub struct TailwindRepoTemplate {
    pub repo: Repository,
    pub files: Vec<FileEntry>,
    pub commits: Vec<Commit>,
    pub branches: Vec<Branch>,
    pub current_branch: String,
    pub current_path: String,
}

#[derive(Template)]
#[template(path = "tailwind/file.html")]
pub struct TailwindFileTemplate {
    pub repo: Repository,
    pub file: FileContent,
    pub commits: Vec<Commit>,
    pub branches: Vec<Branch>,
    pub current_branch: String,
}

// Bootstrap Templates
#[derive(Template)]
#[template(path = "bootstrap/repo.html")]
pub struct BootstrapRepoTemplate {
    pub repo: Repository,
    pub files: Vec<FileEntry>,
    pub commits: Vec<Commit>,
    pub branches: Vec<Branch>,
    pub current_branch: String,
    pub current_path: String,
}

#[derive(Template)]
#[template(path = "bootstrap/file.html")]
pub struct BootstrapFileTemplate {
    pub repo: Repository,
    pub file: FileContent,
    pub commits: Vec<Commit>,
    pub branches: Vec<Branch>,
    pub current_branch: String,
}

// Route handlers
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"<!DOCTYPE html>
<html>
<head>
    <title>Gitea UI Clone - Select Version</title>
    <style>
        body { font-family: system-ui; background: #1a1a2e; color: #eee; display: flex; justify-content: center; align-items: center; min-height: 100vh; margin: 0; }
        .container { text-align: center; }
        h1 { margin-bottom: 2rem; }
        .versions { display: flex; gap: 1rem; flex-wrap: wrap; justify-content: center; }
        a { background: #16213e; color: #fff; padding: 1.5rem 3rem; border-radius: 8px; text-decoration: none; transition: all 0.3s; }
        a:hover { background: #0f3460; transform: translateY(-2px); }
    </style>
</head>
<body>
    <div class="container">
        <h1>Gitea UI Clone</h1>
        <p>Select a CSS framework version:</p>
        <div class="versions">
            <a href="/daisyui/lhumina_research/goose">DaisyUI</a>
            <a href="/tailwind/lhumina_research/goose">Tailwind CSS</a>
            <a href="/bootstrap/lhumina_research/goose">Bootstrap</a>
        </div>
    </div>
</body>
</html>"#)
}

// DaisyUI routes
pub async fn daisyui_repo(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (owner, name) = path.into_inner();
    let repo = get_repository(&owner, &name).unwrap_or_else(|| create_default_repo(&owner, &name));
    let files = load_files("");
    let commits = load_commits();
    let branches = load_branches();

    let template = DaisyRepoTemplate {
        repo,
        files,
        commits,
        branches,
        current_branch: "main".to_string(),
        current_path: String::new(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

pub async fn daisyui_file(path: web::Path<(String, String, String)>) -> Result<HttpResponse> {
    let (owner, name, file_path) = path.into_inner();
    let repo = get_repository(&owner, &name).unwrap_or_else(|| create_default_repo(&owner, &name));
    let file = load_file_content(&file_path).unwrap_or_else(|| create_default_file(&file_path));
    let commits = load_commits();
    let branches = load_branches();

    let template = DaisyFileTemplate {
        repo,
        file,
        commits,
        branches,
        current_branch: "main".to_string(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

// Tailwind routes
pub async fn tailwind_repo(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (owner, name) = path.into_inner();
    let repo = get_repository(&owner, &name).unwrap_or_else(|| create_default_repo(&owner, &name));
    let files = load_files("");
    let commits = load_commits();
    let branches = load_branches();

    let template = TailwindRepoTemplate {
        repo,
        files,
        commits,
        branches,
        current_branch: "main".to_string(),
        current_path: String::new(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

pub async fn tailwind_file(path: web::Path<(String, String, String)>) -> Result<HttpResponse> {
    let (owner, name, file_path) = path.into_inner();
    let repo = get_repository(&owner, &name).unwrap_or_else(|| create_default_repo(&owner, &name));
    let file = load_file_content(&file_path).unwrap_or_else(|| create_default_file(&file_path));
    let commits = load_commits();
    let branches = load_branches();

    let template = TailwindFileTemplate {
        repo,
        file,
        commits,
        branches,
        current_branch: "main".to_string(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

// Bootstrap routes
pub async fn bootstrap_repo(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (owner, name) = path.into_inner();
    let repo = get_repository(&owner, &name).unwrap_or_else(|| create_default_repo(&owner, &name));
    let files = load_files("");
    let commits = load_commits();
    let branches = load_branches();

    let template = BootstrapRepoTemplate {
        repo,
        files,
        commits,
        branches,
        current_branch: "main".to_string(),
        current_path: String::new(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

pub async fn bootstrap_file(path: web::Path<(String, String, String)>) -> Result<HttpResponse> {
    let (owner, name, file_path) = path.into_inner();
    let repo = get_repository(&owner, &name).unwrap_or_else(|| create_default_repo(&owner, &name));
    let file = load_file_content(&file_path).unwrap_or_else(|| create_default_file(&file_path));
    let commits = load_commits();
    let branches = load_branches();

    let template = BootstrapFileTemplate {
        repo,
        file,
        commits,
        branches,
        current_branch: "main".to_string(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

fn create_default_repo(owner: &str, name: &str) -> Repository {
    Repository {
        owner: owner.to_string(),
        name: name.to_string(),
        description: "an open source, extensible AI agent that goes beyond code suggestions".to_string(),
        is_mirror: true,
        mirror_url: "https://github.com/block/goose.git".to_string(),
        synced_ago: "13 minutes ago".to_string(),
        stars: 0,
        forks: 0,
        watchers: 6,
        commits: 2955,
        branches: 968,
        tags: 184,
        size: "2.4 GiB".to_string(),
        default_branch: "main".to_string(),
        topics: vec!["hacktoberfest".to_string(), "mcp".to_string()],
        ssh_url: format!("ssh://git@forge.ourworld.tf/{}/{}.git", owner, name),
        https_url: format!("https://forge.ourworld.tf/{}/{}.git", owner, name),
        website: "https://block.github.io/goose/".to_string(),
    }
}

fn create_default_file(path: &str) -> FileContent {
    FileContent {
        path: path.to_string(),
        name: path.split('/').last().unwrap_or(path).to_string(),
        content: "// File content not found".to_string(),
        language: "rust".to_string(),
        lines: 1,
        size: "0 B".to_string(),
    }
}

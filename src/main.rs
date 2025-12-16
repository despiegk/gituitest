mod data;
mod models;
mod routes;

use actix_files::Files;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Gitea UI Clone server at http://127.0.0.1:8080");
    println!("Available versions:");
    println!("  - DaisyUI:   http://127.0.0.1:8080/daisyui/lhumina_research/goose");
    println!("  - Tailwind:  http://127.0.0.1:8080/tailwind/lhumina_research/goose");
    println!("  - Bootstrap: http://127.0.0.1:8080/bootstrap/lhumina_research/goose");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index))
            // DaisyUI routes
            .route(
                "/daisyui/{owner}/{repo}",
                web::get().to(routes::daisyui_repo),
            )
            .route(
                "/daisyui/{owner}/{repo}/src/{file:.*}",
                web::get().to(routes::daisyui_file),
            )
            // Tailwind routes
            .route(
                "/tailwind/{owner}/{repo}",
                web::get().to(routes::tailwind_repo),
            )
            .route(
                "/tailwind/{owner}/{repo}/src/{file:.*}",
                web::get().to(routes::tailwind_file),
            )
            // Bootstrap routes
            .route(
                "/bootstrap/{owner}/{repo}",
                web::get().to(routes::bootstrap_repo),
            )
            .route(
                "/bootstrap/{owner}/{repo}/src/{file:.*}",
                web::get().to(routes::bootstrap_file),
            )
            // Static files
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
